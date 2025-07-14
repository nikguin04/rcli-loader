use std::{io::Write, sync::{Arc, Mutex}, vec::Vec};

use crate::loading_element::LoadingElement;


pub struct LoadingDrawer {
    list: Vec<Arc<Mutex<LoadingElement>>>
}
impl LoadingDrawer {
    pub fn new() -> LoadingDrawer {
        LoadingDrawer { 
            list: (Vec::new())
        }
    }
    pub fn add_loading_element(&mut self, l_elem: Arc<Mutex<LoadingElement>>) {
        self.list.push(l_elem);
    }

    pub fn draw_at_top(&self) {
        for (i, elem) in self.list.iter().enumerate() {
            print!("\x1B[{line};{column}H", line=i+1, column=0);
            let elem_l = elem.lock().unwrap();
            let progress: i32 = elem_l.get_progress();
            let max: i32 = elem_l.get_max();
            print!("{}/{}", progress, max);

            print!("\x1B[0K"); // Erase from cursor to end of line
            std::io::stdout().flush().unwrap(); // Flush all commands, since no new line is written
        }
    }
}