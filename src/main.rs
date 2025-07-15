use std::{sync::{Arc, Mutex}, thread, time};

use crate::{example_load::sim_load, loading_drawer::LoadingDrawer, loading_element::LoadingElement};
mod terminal_helper;
mod loading_element;
mod loading_drawer;
mod example_load;

fn main() {
    let mut ld: LoadingDrawer = LoadingDrawer::new();

    let le1= Arc::from(Mutex::from(LoadingElement::new(100)));
    let le2= Arc::from(Mutex::from(LoadingElement::new(300)));
    let le3= Arc::from(Mutex::from(LoadingElement::new(1000)));

    ld.add_loading_element(le1.clone());
    ld.add_loading_element(le2.clone());
    ld.add_loading_element(le3.clone());

    sim_load(le1.clone(), 50);
    sim_load(le2.clone(), 25);
    sim_load(le3.clone(), 100);

    println!("Starting loop");
    for _i in 0..600 {
        ld.draw_at_top();
        thread::sleep(time::Duration::from_millis(100));
    }

    //get_terminal_size();
}
