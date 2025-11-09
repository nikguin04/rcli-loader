
use std::{io::{stdin, Read}, pin::Pin, process::exit, sync::{Arc, Mutex, MutexGuard}, task::{Context, Poll, Waker}, thread::{self, sleep}, time::Duration};
use crossterm::{event::KeyModifiers, terminal::{disable_raw_mode, enable_raw_mode}};
use crossterm::event::{self, Event, KeyCode};
use tokio::runtime::Runtime;
use crate::engine::{loading_handler::{rcli_print, LoadingHandler}};
use std::future::Future;

impl LoadingHandler {
    
    pub fn start_stdin_engine(&mut self) {
        let stdin_buffer: Arc<Mutex<String>> = self.data.stdin_buffer.clone();
        thread::spawn(move || {
            loop {
                if event::poll(Duration::from_millis(50)).unwrap() { // Poll for any arrow keys pressed TODO: Make this blocking and polling!
                    if let Event::Key(key_event) = event::read().unwrap() {
                        let mut in_buf_lock = stdin_buffer.lock().unwrap();
                        let ctrl_press = key_event.modifiers.contains(KeyModifiers::CONTROL);
                        if key_event.is_release() { continue; } // Dont handle releases
                        match key_event.code {
                            KeyCode::Left => rcli_print(String::from("Left")),
                            KeyCode::Right => rcli_print(String::from("Right")),
                            KeyCode::Backspace =>  {
                                LoadingHandler::handle_backspace(&mut in_buf_lock, ctrl_press);
                            },
                            KeyCode::Enter => {
                                // TODO: Pop event
                            },
                            KeyCode::Char(c) => { // Add character to buffer
                                if ctrl_press && c == 'c' { // Special case for handling CTRL+C for exiting
                                    println!("Ctrl-C pressed, exiting");
                                    exit(0);
                                }

                                in_buf_lock.push(c);
                            }
                            _ => {},
                        }
                    } 
                }

                thread::sleep(Duration::from_millis(2));
            }
        });
    }

    fn handle_backspace(in_buffer_locked: &mut MutexGuard<'_, String>, ctrl_press: bool) {
        if !ctrl_press {
            in_buffer_locked.pop();
        } else {
            let len = in_buffer_locked.rfind(
                |c: char| -> bool {
                    match c {
                        ' ' | '=' | ':' => true,
                        _ => false
                    }
                }
            ).unwrap_or(0);
            in_buffer_locked.truncate(len);
        }
    }


    pub fn set_stdin_mode(&mut self, enabled: bool) {
        if enabled { // TODO: handle panic
            enable_raw_mode().unwrap();
        } else {
            disable_raw_mode().unwrap();
        }
        self.data.stdin_enabled = enabled;
    }

    pub fn handle_stdin_tick(&mut self) {
        let bufclone = self.data.stdin_buffer.clone();
        let mut stdin_buffer = bufclone.lock().unwrap();
        let split: Vec<&str> = stdin_buffer.split(|c| c == '\r').collect(); // Split as carriage return, it seems raw terminal mode prints \r instead of \n
        if split.len() == 1 { // In this case, no newline/enter is present, we will return as user does not want to execute any command yet
            return;
        }
        
        let returned_line = &split[..split.len()-1].get(0);
        let mut _drain_len: Option<usize> = None; // This produces a "unused" warning, hence the underscore
        match returned_line { 
            None => { return; }
            Some(line) => {
                _drain_len = Some(line.chars().count()+1); // WARNING might need a +1 beacuse of \r
                rcli_print(String::from(**line)); // Temporary
                let stdin_future = self.data.stdin_input_future_state.lock().unwrap();
                match &*stdin_future {
                    None => {}, // TODO: This should execute any regular commands
                    Some(future) => {
                        let mut futurelock = future.lock().unwrap();
                        futurelock.stdin_str = Some(String::from(**line));
                        if let Some(waker) = futurelock.waker.take() { // WARNING: This might break as i dont know the waker and take functionality, this might block anything else from acessing waker
                            waker.wake();
                        }
                    }
                };
            }
        }
        match _drain_len { // This is not in the match as it would cause a mutable borrow after immutable borrow
            Some(_drain_len) => {
                stdin_buffer.drain(.._drain_len);    
            }
            _ => {}
        }
        
    }

    
}

pub struct StdinHandler {
    pub stdin_input_future_state: Arc<Mutex<Option<Arc<Mutex<StdinState>>>>>, // Same as LoadingData
}

impl StdinHandler {
    #[cfg(feature = "e_tokio")]
    pub fn get_input_blocking(&mut self, input_wanted: String) -> Result<String, &'static str> {
        let future = self.set_input_future_state(input_wanted);
        match future {
            Err(msg) => {rcli_print(String::from(msg)); return Err(msg)}
            Ok(future) => {
                let result: String = Runtime::new().unwrap().block_on::<StdinFuture>(future);
                self.clear_input_future_state().unwrap();
                return Ok(result)
            }
        }
        
        
    }
    pub fn get_input_polling(&mut self, input_wanted: String) -> Result<String, &'static str> {
        let future = self.set_input_future_state(input_wanted);
        match future {
            Err(msg) => {rcli_print(String::from(msg)); return Err(msg)}
            Ok(future) => {
                loop {
                    let lock = future.state.try_lock();
                    match lock {
                        Err(_) => {  }
                        Ok(lock) => {
                            match &lock.stdin_str {
                                None => {  }
                                Some(result) => {
                                    self.clear_input_future_state().unwrap();
                                    return Ok(result.clone())
                                }
                            }
                        }
                    }
                    sleep(Duration::from_millis(20));
                }
            }
        }
        
        
    }
    fn set_input_future_state(&mut self, input_wanted: String) -> Result<StdinFuture, &'static str> {
        let future = StdinFuture::new(input_wanted);
        let stdin_future_occupied: bool = match &*self.stdin_input_future_state.lock().unwrap() { None => false, Some(_) => true };
        if stdin_future_occupied {
            return Err("Error getting input, another input request already exists!")
        } else {
            // TODO: use input
            *(self.stdin_input_future_state.lock().unwrap()) = Some(future.state.clone()); // This is synced with loading handlers stdin tick
            return Ok(future)
        }
        
    }

    fn clear_input_future_state(&mut self) -> Result<(), &'static str> {
        *(self.stdin_input_future_state.lock().unwrap()) = None; // This is synced with loading handlers stdin tick
        return Ok(());
    }   
}

pub struct StdinFuture {
    state: Arc<Mutex<StdinState>>,
}
pub struct StdinState {
    stdin_str: Option<String>, // Stdin_str is provided by the handle_stdin_tick, is none, no input yet, otherwise, we have input
    waker: Option<Waker>,
    pub input_wanted: String
}

impl StdinFuture {
    pub fn new(input_wanted: String) -> StdinFuture {
        StdinFuture {
            state: Arc::from(Mutex::from(StdinState {
                stdin_str: None, waker: None, input_wanted: input_wanted
            }))
        }
    }
}
impl Future for StdinFuture {
    type Output = String;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut state = self.state.lock().unwrap();
        return match &state.stdin_str {
            Some(input) => {
                Poll::Ready(input.clone())
            },
            None => {
                state.waker = Some(cx.waker().clone());
                Poll::Pending
            }
        }
    }
}