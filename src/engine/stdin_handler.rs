
use std::{fmt::Debug, io::{stdin, Read, Stdin}, ops::Deref, pin::Pin, process::exit, sync::{Arc, Mutex}, task::{Context, Poll, Waker}, thread, time::Duration};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use tokio::runtime::Runtime;
use crate::engine::{loading_handler::{rcli_print, LoadingHandler}};
use std::future::Future;

impl LoadingHandler {
    
    pub fn start_stdin_engine(&mut self) {
        let stdin_buffer: Arc<Mutex<String>> = self.data.stdin_buffer.clone();
        thread::spawn(move || {
            loop {
                let mut stdin = stdin().lock();
                let mut buffer = [0; 512];
                let mut read: usize;
                loop {
                    read = stdin.read(&mut buffer[..]).unwrap_or(0);
                    if read == 0 { break; }
                    let mut in_buf_lock = stdin_buffer.lock().unwrap();
                    LoadingHandler::handle_static_input_commands(&buffer);
                    in_buf_lock.push_str(str::from_utf8(&buffer[..read]).unwrap()); // TODO: WARNING: This has caused a crash when unwrapping! Wont fix yet as i want to reproduce it
                }
                thread::sleep(Duration::from_millis(2));
            }
        });
    }

    fn handle_static_input_commands(input_buffer: &[u8]) {
        if input_buffer.contains(&0x03) { // Contains Ctrl-C
            println!("Ctrl-C pressed, exiting");
            exit(0);
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
        // println!("{:?}", stdin_buffer.as_bytes());
        let split: Vec<&str> = stdin_buffer.split(|c| c == '\r').collect(); // Split as carriage return, it seems raw terminal mode prints \r instead of \n
        // println!("{:?}", split);
        if split.len() == 1 { // In this case, no newline/enter is present, we will return as user does not want to execute any command yet
            return;
        }
        
        let last: String = split.last().unwrap().to_string(); // Need to duplicate the last element to drop split (minor inefficiency)
        let returned_line = &split[..split.len()-1].get(0);
        match returned_line {
            None => { return; }
            Some(line) => {
                rcli_print(String::from(**line));
            }
        }
        // for elem in &split[..split.len()-1] {
        //     if elem.len() == 0 { continue; }
        //     let mut split_ws = elem.split_whitespace();
        //     let first = split_ws.next().unwrap();
        //     match first {
        //         "test" => {
        //             let mut fmt = String::new();
        //             split_ws.for_each( |e| { fmt.push_str(format!("{}, ", e).as_str())}); // Note: Here this first element (command) should already be skipped
        //             rcli_print(format!("Executed the test command! {:?}", fmt ))
        //         },
        //         _ => { rcli_print(format!("Command not found: {}", first.to_string())); }
        //     }
        // };
        
        stdin_buffer.clear();
        stdin_buffer.push_str(&last);
    }

    
}

pub struct StdinHandler {
    pub stdin_input_future_state: Arc<Mutex<Option<Arc<Mutex<StdinState>>>>>, // Same as LoadingData
}

impl StdinHandler {
    // TODO:  Make cfg e_tokio
    pub fn get_input(&mut self, input_wanted: String) -> Result<String, &'static str> {
        let future = StdinFuture::new(input_wanted);
        match &self.stdin_input_future_state.lock().unwrap().into() {
            Some(_x) => {
                rcli_print(String::from("Error getting input, another input request already exists! returning empty string"));
                return Err("")
            }
            None => {
                // TODO: use input
                *(self.stdin_input_future_state.lock().unwrap()) = Some(future.state.clone()); // This is synced with loading handlers stdin tick
            }
        }
        
        
        let result: String = Runtime::new().unwrap().block_on::<StdinFuture>(future);
        return Ok(result)
    }
}

pub struct StdinFuture {
    state: Arc<Mutex<StdinState>>,
}
pub struct StdinState {
    stdin_str: Option<String>, // Stdin_str is provided by the handle_stdin_tick, is none, no input yet, otherwise, we have input
    waker: Option<Waker>,
    input_wanted: String
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