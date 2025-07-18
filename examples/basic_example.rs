use std::{sync::{Arc, RwLock}, time::{self, Duration}};


mod modules;
use rcli_loader::{loading_drawer::LoadingDrawer, loading_element::LoadingElement};
use tokio::time::sleep;

use crate::modules::{example_download::sim_download, example_load::sim_load};

#[tokio::main]
async fn main() {
    let mut ld: LoadingDrawer = LoadingDrawer::new();

    let le1= Arc::from(RwLock::from(LoadingElement::new(100)));
    let le2= Arc::from(RwLock::from(LoadingElement::new(300)));
    let le3= Arc::from(RwLock::from(LoadingElement::new(1000)));
    let le4= Arc::from(RwLock::from(LoadingElement::empty()));

    ld.add_loading_element(le1.clone());
    ld.add_loading_element(le2.clone());
    ld.add_loading_element(le3.clone());
    ld.add_loading_element(le4.clone());

    sim_load(le1.clone(), 50);
    sim_load(le2.clone(), 25);
    sim_load(le3.clone(), 100);
    sim_download(le4.clone());

    println!("Starting loop");
    for _i in 0..600 {
        ld.draw_at_top();
        sleep(Duration::from_millis(100)).await;
    }

    //get_terminal_size();
}
