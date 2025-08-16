use std::{collections::VecDeque, io::Write, sync::{Arc, Mutex, MutexGuard, OnceLock, RwLock}, time::Duration, vec::Vec};

use crate::{drawer_helper::{print_splitter_line, set_terminal_pos, LoadingColorScheme, Position}, loading_element::LoadingElement, terminal_helper::{get_terminal_size, V2Usz}};
use tokio::task::JoinHandle;
use tokio::time::sleep;

const PROGRESS_CHARS_COUNT: u8 = 8;
static PROGRESS_CHARS: &'static [char] = &['\u{258F}', '\u{258E}', '\u{258D}', '\u{258C}', '\u{258B}', '\u{258A}', '\u{2589}', '\u{2588}'];
static _LOADING_DRAWER: OnceLock<Mutex<LoadingDrawer>> = OnceLock::new();
// Macro to define LOADING_DRAWER as mutex lock from _LOADING_DRAWER

pub struct LoadingDrawer {
    list: Vec<Arc<RwLock<LoadingElement>>>,
    color_scheme: Option<Box<dyn LoadingColorScheme + Send + Sync>>,
    print_history: VecDeque<String>,
    loadingbar_anchor_position: Position,
    max_history: usize,
    stdin_enabled: bool,
    allocated_rows_loadingbars: usize
}
impl LoadingDrawer {
    fn get_remaining_height(&self) -> usize { // WARNING: Make sure that only one drawable uses this function, as only one drawable can take up the "rest" of the space
        return get_terminal_size().y 
            - self.list.len()
            - (if self.stdin_enabled {3} else {0});
    }
    pub fn set_colorscheme(&mut self, color_scheme: Box<dyn LoadingColorScheme + Send + Sync>) {
        self.color_scheme = Some(color_scheme);
    }
    pub fn set_loadingbar_anchor_position(&mut self, position: Position) {
        self.loadingbar_anchor_position = position; // TODO: Figure if this should also redraw everything, or if we assume that automatically happens
    }
    pub fn add_loading_element(&mut self, l_elem: Arc<RwLock<LoadingElement>>) {
        self.list.push(l_elem);
    }
    pub fn set_stdin_mode(&mut self, enabled: bool) {
        self.stdin_enabled = enabled;
    }

    pub fn start_draw_loop(&mut self) -> JoinHandle<()> {
        tokio::spawn(async {
            println!("Starting loop");
            for _i in 0..600 {
                draw_all();
                rcli_print(format!("line {}\n", _i));
                if _i % 20 == 0 { rcli_print(format!("Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.")) };
                sleep(Duration::from_millis(100)).await;
            }
        })
    }
}

#[allow(private_interfaces)]
pub fn get_loading_drawer() -> MutexGuard<'static, LoadingDrawer> {
    _LOADING_DRAWER.get_or_init(||
        Mutex::new(
            LoadingDrawer { 
                list: (Vec::new()),
                color_scheme: None,
                print_history: VecDeque::new(),
                loadingbar_anchor_position: Position::BOTTOM, // TODO: Make setter
                max_history: 50, // TODO: Make setter
                stdin_enabled: true,
                allocated_rows_loadingbars: 5 // TODO: Make dynamically adjust, or change by setter
            }
        )
    ).lock().unwrap()
}

pub fn erase_screen() { // Usually to be used at init
    println!("\x1B[2J");
}
pub fn hide_cursor() { // Implementation specific for consoles, might not work
    println!("\x1b[?25l");
}
pub fn show_cursor() { // Implementation specific for consoles, might not work
    println!("\x1b[?25h");
}

pub fn rcli_print(print_str: String) {
    let mut drawer: MutexGuard<'static, LoadingDrawer> = get_loading_drawer();
    if drawer.print_history.len() > drawer.max_history { drawer.print_history.pop_back(); } // Keep history at constant/max size
    drawer.print_history.push_front(print_str);
    drop(drawer);
    draw_print_history();
}

// Future todo note: When making scrolling behaviour, slice the messages whenever window is resized and when a new message is added, so they will be presliced for printing.
pub fn draw_print_history() {
    let drawer: MutexGuard<'static, LoadingDrawer> = get_loading_drawer();
    let history: &VecDeque<String> = &drawer.print_history;
    let sz: V2Usz = get_terminal_size();
    let pos: &Position = &drawer.loadingbar_anchor_position;

    let offset: usize = match pos { // Offset height for printing history, which our terminal cursor must jump to, as to avoid overwriting loading bars
        Position::BOTTOM => 0,
        Position::TOP => drawer.list.len() + 1
    };
    if sz.y <= drawer.list.len() + 1 { // Accounting for both loading elemenets and splitter line
        println!("\x1b[1EWindow is too small to print history\x1b[0K"); // Reset cursor to next line and foribly print error, also clear to end of line
        return;
    }
    let mut remaining_height: usize = drawer.get_remaining_height();

    print_splitter_line(&sz, match pos { Position::BOTTOM => remaining_height, Position::TOP => offset }); // Print either at top or bottom of message "box" depending on the wanted position anchoring
    remaining_height -= 1;

    // Iteratte over each history element, TODO: feature: this should be line indexed already so we can scroll up, and should not just start at first history element and line
    'outer: for prt_stmnt in history.iter() { // Note: due to vecdeque, we already iterate from front to back
        for line in prt_stmnt.lines().rev() {
            for term_fit_line in line.as_bytes().chunks(sz.x as usize - 1).rev() { // Chunk every line so that we can calculate how many times one "line" would wrap in our console, and adjust the remaining height.
                set_terminal_pos(V2Usz { x: 0, y: offset + remaining_height });
                print!("{}\x1b[0K", std::str::from_utf8(term_fit_line).unwrap()); // Convert line back to string or utf8, and clear "rest of line"
                remaining_height -= 1;
                if remaining_height == 0 { break 'outer; } // Note: Should this flush as well?
            };
        };
    };

    std::io::stdout().flush().unwrap();
}

pub fn draw_all() {
    draw_loader();
    draw_print_history();
    //draw_input_area();
}

fn draw_loader() {
    let sz: V2Usz = get_terminal_size();
    let drawer = get_loading_drawer();
    for (i, elem) in drawer.list.iter().enumerate() {
        let line = match drawer.loadingbar_anchor_position {
            Position::TOP => i+1,
            Position::BOTTOM => sz.y as usize - i // This effectively reverses position of queue when printed
        };
        print!("\x1B[{line};{column}H", line=line, column=0);
        // Minus with two as that the reported screen size is two chars too big and will wrap. WARNING: Can cause errors if screen size is below 2 width?
        let mut unused_char_count: usize = sz.x as usize - 2; // Defines as usize, as all of the string.len() returns usize, so no bulky conversions later

        // All work on element to release read lock quickly
        let elem_l = elem.read().unwrap();
        let progress: usize = elem_l.get_progress();
        let max: usize = elem_l.get_max();
        let decimal_progress: f32 = elem_l.get_progress_decimal() as f32; // No reason to store and work on f64, since we do not need that precision here
        let name: Arc<Box<str>> = elem_l.get_name();
        
        // Prepare strings to be printed
        let progress_chunks_str: String = format!("{progress}/{max} ", // Format the progress first, so we can release elem_l
                progress=elem_l.format_progress_unit(progress),
                max=elem_l.format_progress_unit(max)); 
        let name_str: String = format!("{}: ", name);

        
        // Printout before char loading block
        print!("{}", name_str);
        print!("{progress}", progress = progress_chunks_str);
        
        
        // Update unused character count accorind to everyting printed, and what we expect to print (excpet for block char loading) 
        unused_char_count -= name.len(); // TODO: Error (attempt to subtract with overflow), if screen is not big enough
        unused_char_count -= progress_chunks_str.len();

        // Print progress char blocks, after everything has been printed, except for the block char loading
        let pct_per_char: f32 = 1.0 / unused_char_count as f32;
        let endchar: char = PROGRESS_CHARS[ ( (decimal_progress%pct_per_char) / pct_per_char * PROGRESS_CHARS_COUNT as f32 ) as usize ];
        let fillchar_len: usize = (decimal_progress / pct_per_char) as usize;
        match &drawer.color_scheme {
            None => print!("{endchar:\u{2588}>fillchar_len$}", endchar = endchar, fillchar_len = fillchar_len ),
            Some(x) => print!("{col_start}{endchar:\u{2588}>fillchar_len$}\x1b[0m",  endchar = endchar, fillchar_len = fillchar_len, col_start = x.get_char_block_color(&elem_l))
        }
        
        //rcli_print!("test\n{}", "123");
        

        print!("\x1B[0K"); // Erase from cursor to end of line (Only necessary when whole line is not written!)
        std::io::stdout().flush().unwrap(); // Flush all commands, since no new line is written
    }
}

