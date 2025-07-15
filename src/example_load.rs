use std::{sync::{Arc, Mutex}, thread, time::Duration};

use crate::loading_element::LoadingElement;



pub fn sim_load(loading_element: Arc<Mutex<LoadingElement>>, delay: u64) {
    thread::spawn(move || {
        let max: usize = loading_element.lock().unwrap().get_max();
        for _i in 0..max {
            loading_element.lock().unwrap().update(1);
            thread::sleep(Duration::from_millis(delay));
        }
    });

}