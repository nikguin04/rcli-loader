use std::{ops::DerefMut, sync::{Arc, RwLock}, thread, time::Duration};
use humansize::{format_size, DECIMAL};

mod modules;
use rcli_loader::{drawing::drawer_helper::{erase_screen, hide_cursor, RedGreenScheme}, engine::loading_handler::{rcli_print, spawn_loader_engine, LOADING_HANDLER, STDIN_HANDLER}, structure::loading_element::LoadingElement};
use tokio::time::sleep;

use crate::modules::{example_download::sim_download, example_load::sim_load};

#[tokio::main]
async fn main() {
    erase_screen();
    hide_cursor();
    
    let mut handler = LOADING_HANDLER.lock().unwrap();
    //let mut drawer = handler.drawer;
    handler.drawer.set_colorscheme(Box::from(RedGreenScheme {}));

    

    // TODO: Make a ::new functions for this
    let le1= Arc::from(RwLock::from(LoadingElement::new(100, Box::from("Loader 1"), None )));
    let le2= Arc::from(RwLock::from(LoadingElement::new(300, Box::from("Loader 2"), None )));
    let le3= Arc::from(RwLock::from(LoadingElement::new(1000, Box::from("Loader 123"), None )));
    let convert_function: fn(usize) -> Box<str> = convert_byte;
    let le4= Arc::from(RwLock::from(LoadingElement::new(0, Box::from("Big Buck Bunny"), Some(convert_function) ))); // TODO: make defaulting max values

    
    handler.add_loading_element(le1.clone());
    handler.add_loading_element(le2.clone());
    handler.add_loading_element(le3.clone());
    handler.add_loading_element(le4.clone());

    
    sim_load(le1.clone(), 50);
    sim_load(le2.clone(), 25);
    sim_load(le3.clone(), 100);
    sim_download(le4.clone());
    drop(handler);

    spawn_loader_engine();
    
    sleep(Duration::from_millis(2000)).await;
    thread::spawn(|| { // Get some random input here
        let mut stdin_handler = STDIN_HANDLER.lock().unwrap();
        let input = stdin_handler.get_input(String::from("Give me a banana: "));
        match input {
            Ok (input) =>  {
                if input.to_lowercase().contains("banana") {
                    rcli_print(String::from("Thanks for the banana!"));
                } else {
                    rcli_print(String::from("I did not get a banana!"));
                }
            }
            Err(_) => {rcli_print(String::from("Error getting input"));}
        } 
    });
    
    while (true) {
        sleep(Duration::from_millis(100)).await;
    }
    
}

fn convert_byte(value: usize) -> Box<str> {
    Box::from(format_size(value, DECIMAL))
}
