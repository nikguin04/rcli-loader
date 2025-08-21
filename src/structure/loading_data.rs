use std::{collections::VecDeque, sync::{Arc, Mutex, RwLock}};

use crate::structure::loading_element::LoadingElement;



pub struct LoadingData {
    pub list: Vec<Arc<RwLock<LoadingElement>>>,
    pub print_history: VecDeque<String>,
    pub print_buffer: Arc<Mutex<VecDeque<String>>>,
    pub max_history: usize,
    pub stdin_enabled: bool,
}

impl LoadingData {
    pub fn flush_print_buffer(&mut self) {
        let mut buf = self.print_buffer.lock().unwrap();
        while !buf.is_empty() {
            if self.print_history.len() >= self.max_history { self.print_history.pop_back(); }
            self.print_history.push_front(buf.pop_front().unwrap_or(String::from("ERROR WHEN FLUSHING PRINT BUFFER!")));
        }
    }
}