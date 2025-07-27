use std::{io::Write, sync::{Arc, Mutex, MutexGuard, OnceLock, RwLock}, vec::{Vec}};


use crate::{drawer_helper::LoadingColorScheme, loading_element::LoadingElement, terminal_helper::{get_terminal_size, C2U16}};

const PROGRESS_CHARS_COUNT: u8 = 8;
static PROGRESS_CHARS: &'static [char] = &['\u{258F}', '\u{258E}', '\u{258D}', '\u{258C}', '\u{258B}', '\u{258A}', '\u{2589}', '\u{2588}'];
static _LOADING_DRAWER: OnceLock<Mutex<LoadingDrawer>> = OnceLock::new();
// Macro to define LOADING_DRAWER as mutex lock from _LOADING_DRAWER

struct LoadingDrawer {
    list: Vec<Arc<RwLock<LoadingElement>>>,
    color_scheme: Option<Box<dyn LoadingColorScheme + Send + Sync>>
}
#[allow(private_interfaces)]
fn get_loading_drawer() -> MutexGuard<'static, LoadingDrawer> {
    _LOADING_DRAWER.get_or_init(||
        Mutex::new(
            LoadingDrawer { 
                list: (Vec::new()),
                color_scheme: None
            }
        )
    ).lock().unwrap()
}
pub fn set_colorscheme(color_scheme: Box<dyn LoadingColorScheme + Send + Sync>) {
    get_loading_drawer().color_scheme = Some(color_scheme);
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

pub fn add_loading_element(l_elem: Arc<RwLock<LoadingElement>>) {
    get_loading_drawer().list.push(l_elem);
}


pub fn rcli_print(print_str: &String) { // TODO: Currently hardcoded for bottom position, make for top aswell
    let line_count = print_str.lines().count();
    let tsize: C2U16 = get_terminal_size();
    // TODO: Will have errors if drawer count is bigger than screen size plus the line count
    print!("\x1b[{y};{x}H", x = 0, y = tsize.y as usize - _LOADING_DRAWER.get().iter().count() - line_count - 2); // Set cursor position to height of top line
    print!("{}", print_str);
    // Set cursor position to bottom and Fill with newlines
    print!("\x1b[{y};{x}H{endchar:\n>fillchar_len$}", x=0, y=usize::MAX, endchar="", fillchar_len=line_count);
}

pub enum Position {
    TOP,BOTTOM
}
pub fn draw_loader(position: Position) {
    let sz: C2U16 = get_terminal_size();
    let drawer = get_loading_drawer();
    for (i, elem) in drawer.list.iter().enumerate() {
        let line = match position {
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
        

        //print!("\x1B[0K"); // Erase from cursor to end of line (Only necessary when whole line is not written!)
        std::io::stdout().flush().unwrap(); // Flush all commands, since no new line is written
    }
}

