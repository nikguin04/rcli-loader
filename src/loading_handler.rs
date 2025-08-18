

use std::{collections::VecDeque, rc::Rc, sync::{Arc, Mutex, RwLock}, thread::{self}, time::Duration};

use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use lazy_static::lazy_static;
use tokio::{task::JoinHandle, time::sleep};

use crate::{drawer_helper::LoadingColorScheme, loading_data::LoadingData, loading_drawer::LoadingDrawer, loading_element::LoadingElement, terminal_helper::get_terminal_size};

lazy_static! {
    static ref PRINT_BUFFER: Arc<Mutex<VecDeque<String>>> = Arc::from(Mutex::from(VecDeque::new()));
    pub static ref LOADING_HANDLER: Mutex<LoadingHandler> = Mutex::from(LoadingHandler { 
        data: LoadingData {
            list: Vec::new(),
            print_history: VecDeque::new(),
            print_buffer: PRINT_BUFFER.clone(),
            max_history: 50, // TODO: Make setter
            stdin_enabled: true
        },
        drawer: LoadingDrawer::default()
    });
}

pub struct LoadingHandler {
    pub data: LoadingData,
    pub drawer: LoadingDrawer,

}
impl LoadingHandler {
    pub fn add_loading_element(&mut self, l_elem: Arc<RwLock<LoadingElement>>) {
        self.data.list.push(l_elem);
    }
    pub fn set_stdin_mode(&mut self, enabled: bool) {
        if enabled { enable_raw_mode().unwrap(); } else { disable_raw_mode().unwrap(); } // TODO: handle panic
        self.data.stdin_enabled = enabled;
    }

    fn handle_loader_tick(&mut self) {
        self.drawer.draw_all(&mut self.data);
        //self.handle_stdin();
    }

    pub fn start_loader_engine(&mut self) {
        println!("Starting loop");
        for _i in 0..600 {
            self.handle_loader_tick();
            rcli_print(format!("line {}\n", _i));
            if _i % 20 == 0 { rcli_print(format!("Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.")) };
            thread::sleep(Duration::from_millis(100));
        }
    }
    pub fn spawn_loader_engine(mut self) -> JoinHandle<()> {
        //drop(self); // We need to make sure that we take ownership of the loading drawer, to then lock it again in the new async tgread
        return tokio::spawn(async move {
            //let drawer = get_loading_drawer();
            println!("Starting loop");
            for _i in 0..600 {
                self.handle_loader_tick();
                rcli_print(format!("line {}\n", _i));
                if _i % 20 == 0 { rcli_print(format!("Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.")) };
                sleep(Duration::from_millis(100)).await;
            }
        })
    }

    fn init(&mut self) {
        self.set_stdin_mode(self.data.stdin_enabled); // Trigger the stdin raw mode (or not)
    }
}




pub fn rcli_print(print_str: String) {
    let mut buf = PRINT_BUFFER.lock().unwrap();
    buf.push_back(print_str);
}