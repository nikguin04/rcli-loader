use std::{collections::VecDeque, io::Write, sync::{Arc, Mutex, RwLock}, thread, time::Duration, vec::Vec};

use crate::{draw_ordering::{DrawOrdering, DrawableElement, DrawableElementFill, DRAW_PRINT_HISTORY, LOADING_BAR}, drawer_helper::{print_splitter_line, set_terminal_pos, LoadingColorScheme, Position}, loading_data::LoadingData, loading_element::LoadingElement, loading_handler::LoadingHandler, terminal_helper::{get_terminal_size, V2Usz}};
const PROGRESS_CHARS_COUNT: u8 = 8;
static PROGRESS_CHARS: &'static [char] = &['\u{258F}', '\u{258E}', '\u{258D}', '\u{258C}', '\u{258B}', '\u{258A}', '\u{2589}', '\u{2588}'];

pub struct LoadingDrawer {
    color_scheme: Option<Box<dyn LoadingColorScheme + Send + Sync>>,
    allocated_rows_loadingbars: usize,
    draw_ordering: DrawOrdering
}

impl LoadingDrawer {
    pub fn default() -> LoadingDrawer {
        LoadingDrawer {
            color_scheme: None,         
            allocated_rows_loadingbars: 5, // TODO: Make dynamically adjust, or change by setter
            draw_ordering: DrawOrdering {
                elements: vec![LOADING_BAR], // TODO: Make input field
                fill_element: DRAW_PRINT_HISTORY
            }
        }
    }
    
    pub fn draw_all(&mut self, data: &mut LoadingData) {
        self.draw_ordering.elements.iter().for_each(|elem| {
            (elem.draw)(self, elem, data);
        });
        (self.draw_ordering.fill_element.draw)(self, &(self.draw_ordering.fill_element), data)
    }

    

    pub fn set_colorscheme(&mut self, color_scheme: Box<dyn LoadingColorScheme + Send + Sync>) {
        self.color_scheme = Some(color_scheme);
    }

    // TODO: We should have both a function for getting height used in top AND bottom, then combine it here. Perhaps keep track of where the elements are with a proper abstract class
    fn get_remaining_height(data: &LoadingData) -> usize { // WARNING: Make sure that only one drawable uses this function, as only one drawable can take up the "rest" of the space
        return get_terminal_size().y 
            - data.list.len()
            - (if data.stdin_enabled {3} else {0});
    }
}
impl LoadingDrawer {
        // Future todo note: When making scrolling behaviour, slice the messages whenever window is resized and when a new message is added, so they will be presliced for printing.
        pub fn draw_print_history(&self, element: &DrawableElementFill, data: &mut LoadingData) {
            data.flush_print_buffer();
            let history: &VecDeque<String> = &data.print_history;
            let sz: V2Usz = get_terminal_size();

            // TODO: FIX OFFSET
            let offset: usize = match Position::BOTTOM { // Offset height for printing history, which our terminal cursor must jump to, as to avoid overwriting loading bars
                Position::BOTTOM => 0,
                Position::TOP => data.list.len() + 1
            };
            if sz.y <= data.list.len() + 1 { // Accounting for both loading elemenets and splitter line
                println!("\x1b[1EWindow is too small to print history\x1b[0K"); // Reset cursor to next line and foribly print error, also clear to end of line
                return;
            }
            let mut remaining_height: usize = LoadingDrawer::get_remaining_height(data);

            // TODO: FIX OFFSET
            print_splitter_line(&sz, match Position::BOTTOM { Position::BOTTOM => remaining_height, Position::TOP => offset }); // Print either at top or bottom of message "box" depending on the wanted position anchoring
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


    pub fn draw_loader(&self, element: &DrawableElement, data: &mut LoadingData) {
        let sz: V2Usz = get_terminal_size();
        for (i, elem) in data.list.iter().enumerate() {
            let line = match element.pos {
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
            match &self.color_scheme {
                None => print!("{endchar:\u{2588}>fillchar_len$}", endchar = endchar, fillchar_len = fillchar_len ),
                Some(x) => print!("{col_start}{endchar:\u{2588}>fillchar_len$}\x1b[0m",  endchar = endchar, fillchar_len = fillchar_len, col_start = x.get_char_block_color(&elem_l))
            }
            
            //rcli_print!("test\n{}", "123");
            

            print!("\x1B[0K"); // Erase from cursor to end of line (Only necessary when whole line is not written!)
            std::io::stdout().flush().unwrap(); // Flush all commands, since no new line is written
        }
    }
}