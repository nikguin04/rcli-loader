

use std::{collections::VecDeque, io::Stdin, sync::{Arc, Mutex, RwLock}, thread::{self}, time::Duration};

use lazy_static::lazy_static;

#[cfg(feature = "e_tokio")]
use tokio::{task::JoinHandle, time::sleep};

use crate::{drawing::loading_drawer::LoadingDrawer, engine::stdin_handler::{StdinHandler, StdinState}, structure::{loading_data::LoadingData, loading_element::LoadingElement}};


lazy_static! {
    static ref PRINT_BUFFER: Arc<Mutex<VecDeque<String>>> = Arc::from(Mutex::from(VecDeque::new()));
    static ref MutStdinStateRef: Arc<Mutex<Option<Arc<Mutex<StdinState>>>>> = Arc::from(Mutex::from(None));
    pub static ref LOADING_HANDLER: Mutex<LoadingHandler> = Mutex::from(LoadingHandler::new(
        LoadingData {
            list: Vec::new(),
            print_history: VecDeque::new(),
            print_buffer: PRINT_BUFFER.clone(),
            max_history: 50, // TODO: Make setter
            stdin_enabled: true,
            stdin_buffer: Arc::from(Mutex::from(String::new())),
            stdin_input_future_state: MutStdinStateRef.clone()
        },
        LoadingDrawer::default()
    ));
    pub static ref STDIN_HANDLER: Mutex<StdinHandler> = Mutex::from(StdinHandler { stdin_input_future_state: MutStdinStateRef.clone() });
}

pub struct LoadingHandler {
    pub data: LoadingData,
    pub drawer: LoadingDrawer
}
impl LoadingHandler {
    fn new(data: LoadingData, drawer: LoadingDrawer) -> LoadingHandler {
        let mut lh = LoadingHandler { data:data, drawer:drawer };
        lh.init();
        lh.start_stdin_engine();
        return lh;
    }
    pub fn add_loading_element(&mut self, l_elem: Arc<RwLock<LoadingElement>>) {
        self.data.list.push(l_elem);
    }

    fn handle_loader_tick(&mut self) {
        self.handle_stdin_tick();
        self.drawer.draw_all(&mut self.data);
    }

    fn init(&mut self) {
        self.set_stdin_mode(self.data.stdin_enabled); // Trigger the stdin raw mode (or not)
    }
}

pub fn start_loader_engine() {
        println!("Starting loop");
        for _i in 0..600 {
            let mut handler = LOADING_HANDLER.lock().unwrap();
            handler.handle_loader_tick();
            //rcli_print(format!("line {}\n", _i));
            //if _i % 20 == 0 { rcli_print(format!("Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.")) };
            thread::sleep(Duration::from_millis(100));
        }
    }

#[cfg(feature = "e_tokio")]
pub fn spawn_loader_engine() -> JoinHandle<()> {
    return tokio::spawn(async move {
        for _i in 0..600 {
            {
                let mut handler = LOADING_HANDLER.lock().unwrap();
                handler.handle_loader_tick();
                //rcli_print(format!("line {}\n", _i));
                //if _i % 20 == 0 { rcli_print(format!("Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.")) };
            } // handler is dropped here
            sleep(tokio::time::Duration::from_millis(100)).await;
        }
    })
}

pub fn rcli_print(print_str: String) {
    let mut buf = PRINT_BUFFER.lock().unwrap();
    buf.push_back(print_str);
}