
use std::{io::{stdin, Read}, sync::{Arc, Mutex}, thread, time::Duration};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use crate::engine::loading_handler::LoadingHandler;

impl LoadingHandler {
    
    pub fn start_stdin_engine(&mut self) {
        let stdin_buffer: Arc<Mutex<String>> = self.data.stdin_buffer.clone();
        thread::spawn(move || {
            let mut lock = stdin_buffer.lock().unwrap();
            loop {
                let mut stdin = stdin().lock();
                let mut buffer = [0; 512];
                while stdin.read(&mut buffer[..]).unwrap_or(0) > 0 {
                    lock.push_str(str::from_utf8(&buffer[..]).unwrap()); // TODO: WARNING: This has caused a crash when unwrapping! Wont fix yet as i want to reproduce it
                }
                thread::sleep(Duration::from_millis(2));
            }
        });
    }

    pub fn set_stdin_mode(&mut self, enabled: bool) {
        if enabled { // TODO: handle panic
            enable_raw_mode().unwrap();
        } else {
            disable_raw_mode().unwrap();
        }
        self.data.stdin_enabled = enabled;
    }
}