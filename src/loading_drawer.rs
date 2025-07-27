use std::{io::Write, sync::{Arc, RwLock}, vec::{self, Vec}};

use crate::{drawer_helper::LoadingColorScheme, loading_element::LoadingElement, terminal_helper::{get_terminal_size, C2U16}};

const PROGRESS_CHARS_COUNT: u8 = 8;
static PROGRESS_CHARS: &'static [char] = &['\u{258F}', '\u{258E}', '\u{258D}', '\u{258C}', '\u{258B}', '\u{258A}', '\u{2589}', '\u{2588}'];

pub struct LoadingDrawer {
    list: Vec<Arc<RwLock<LoadingElement>>>,
    color_scheme: Option<Box<dyn LoadingColorScheme>>
}
impl LoadingDrawer {
    pub fn new() -> LoadingDrawer {
        println!("\x1B[2J"); // Erase entire screen - Might be unnecessary for this new(), but nice for testing right now
        LoadingDrawer { 
            list: (Vec::new()),
            color_scheme: None
        }
    }
    pub fn new_custom(color_scheme: Box<dyn LoadingColorScheme>) -> LoadingDrawer {
        LoadingDrawer {
            list: (Vec::new()),
            color_scheme: Some(color_scheme)
        }
    }
    pub fn add_loading_element(&mut self, l_elem: Arc<RwLock<LoadingElement>>) {
        self.list.push(l_elem);
    }

    pub fn draw_at_top(&self) {
        let sz: C2U16 = get_terminal_size().unwrap(); // TODO: Implement panic handler for drawing at upper function

        for (i, elem) in self.list.iter().enumerate() {
            print!("\x1B[{line};{column}H", line=i+1, column=0);
            let mut unused_char_count: usize = sz.x as usize; // Defines as usize, as all of the string.len() returns usize, so no bulky conversions later

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
            unused_char_count -= name.len();
            unused_char_count -= progress_chunks_str.len();

            // Print progress char blocks, after everything has been printed, except for the block char loading
            let pct_per_char: f32 = 1.0 / unused_char_count as f32;
            let endchar: char = PROGRESS_CHARS[ ( (decimal_progress%pct_per_char) / pct_per_char * PROGRESS_CHARS_COUNT as f32 ) as usize ];
            let fillchar_len: usize = (decimal_progress / pct_per_char) as usize;
            match (&self.color_scheme) {
                None => print!("{endchar:\u{2588}>fillchar_len$}", endchar = endchar, fillchar_len = fillchar_len ),
                Some(x) => print!("{col_start}{endchar:\u{2588}>fillchar_len$}\x1b[0m",  endchar = endchar, fillchar_len = fillchar_len, col_start = x.get_char_block_color(&elem_l))
            }
            
            


            //print!("\x1B[0K"); // Erase from cursor to end of line (Only necessary when whole line is not written!)
            std::io::stdout().flush().unwrap(); // Flush all commands, since no new line is written
        }
    }
}
