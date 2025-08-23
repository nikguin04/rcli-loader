
use std::{io::{stdin, Read}, process::exit, sync::{Arc, Mutex}, thread, time::Duration};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use crate::engine::loading_handler::LoadingHandler;

impl LoadingHandler {
    
    pub fn start_stdin_engine(&mut self) {
        let stdin_buffer: Arc<Mutex<String>> = self.data.stdin_buffer.clone();
        thread::spawn(move || {
            loop {
                let mut stdin = stdin().lock();
                let mut buffer = [0; 512];
                while stdin.read(&mut buffer[..]).unwrap_or(0) > 0 {
                    let mut in_buf_lock = stdin_buffer.lock().unwrap();
                    LoadingHandler::handle_static_input_commands(&buffer);
                    in_buf_lock.push_str(str::from_utf8(&buffer[..]).unwrap()); // TODO: WARNING: This has caused a crash when unwrapping! Wont fix yet as i want to reproduce it
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
        let mut stdin_buffer = self.data.stdin_buffer.lock().unwrap();
        let split: Vec<&str> = stdin_buffer.split("\r").collect(); // Slit as carriage return, it seems raw terminal mode prints \r instead of \n
        if split.len() == 1 { // In this case, no newline/enter is present, we will return as user does not want to execute any command yet
            return;
        }
        let iterator = split.iter().take(split.len()-1);
        iterator.for_each(|elem| {
            
        });
        let last = split.last().unwrap().to_string(); // Need to duplicate the last element to drop split (minor inefficiency)
        stdin_buffer.clear();
        stdin_buffer.push_str(&last);
    }
}