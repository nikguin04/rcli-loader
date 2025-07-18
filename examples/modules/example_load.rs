use std::{sync::{Arc, RwLock}, thread, time::Duration};

use rcli_loader::loading_element::LoadingElement;



pub fn sim_load(loading_element: Arc<RwLock<LoadingElement>>, delay: u64) {
    thread::spawn(move || {
        let max: usize = loading_element.read().unwrap().get_max();
        for _i in 0..max {
            loading_element.write().unwrap().update(1);
            thread::sleep(Duration::from_millis(delay));
        }
    });

}