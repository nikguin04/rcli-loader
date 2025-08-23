
use std::{io::{stdin, Read}, process::exit, sync::{Arc, Mutex}, thread, time::Duration};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use crate::engine::loading_handler::{rcli_print, LoadingHandler};

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
        for elem in &split[..split.len()-1] {
            if elem.len() == 0 { continue; }
            let mut split_ws = elem.split_whitespace();
            let first = split_ws.next().unwrap();
            match first {
                "test" => { rcli_print(format!("Executed the test command! {:?}", split_ws)); },
                _ => { rcli_print(format!("Command not found: {}", first.to_string())); }
            }
        };
        
        stdin_buffer.clear();
        stdin_buffer.push_str(&last);
    }
}