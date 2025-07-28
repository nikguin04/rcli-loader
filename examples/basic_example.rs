use std::{sync::{Arc, RwLock}, time::{self, Duration}};
use humansize::{format_size, DECIMAL};

mod modules;
use rcli_loader::{drawer_helper::RedGreenScheme, loading_drawer::{add_loading_element, draw_loader, erase_screen, hide_cursor, rcli_print, set_colorscheme}, loading_element::LoadingElement};
use tokio::time::sleep;

use crate::modules::{example_download::sim_download, example_load::sim_load};

#[tokio::main]
async fn main() {
    erase_screen();
    hide_cursor();
    set_colorscheme(Box::from(RedGreenScheme {}));

    let le1= Arc::from(RwLock::from(LoadingElement::new(100, Box::from("Loader 1"), None )));
    let le2= Arc::from(RwLock::from(LoadingElement::new(300, Box::from("Loader 2"), None )));
    let le3= Arc::from(RwLock::from(LoadingElement::new(1000, Box::from("Loader 123"), None )));
    let convert_function: fn(usize) -> Box<str> = convert_byte;
    let le4= Arc::from(RwLock::from(LoadingElement::new(0, Box::from("Big Buck Bunny"), Some(convert_function) ))); // TODO: make defaulting max values

    add_loading_element(le1.clone());
    add_loading_element(le2.clone());
    add_loading_element(le3.clone());
    add_loading_element(le4.clone());

    sim_load(le1.clone(), 50);
    sim_load(le2.clone(), 25);
    sim_load(le3.clone(), 100);
    sim_download(le4.clone());

    println!("Starting loop");
    for _i in 0..600 {
        draw_loader();
        rcli_print(format!("line {}\n", _i));
        if _i % 20 == 0 { rcli_print(format!("Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.")) };
        sleep(Duration::from_millis(100)).await;
    }
}

fn convert_byte(value: usize) -> Box<str> {
    Box::from(format_size(value, DECIMAL))
}
