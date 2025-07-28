use crate::{loading_element::LoadingElement, terminal_helper::C2U16};
pub enum Position {
    TOP,BOTTOM
}

pub trait LoadingColorScheme {
    fn get_char_block_color(&self, l_elem: &LoadingElement) -> Box<str>; // WARNING: Return of string as color with ANSI codes is unreliable, this should be made to some kind of struct?
}

pub struct RedGreenScheme {}
impl LoadingColorScheme for RedGreenScheme {
    fn get_char_block_color(&self, l_elem: &LoadingElement) -> Box<str> {
        let ratio = if l_elem.get_max() > 0 {255 * l_elem.get_progress() / l_elem.get_max()} else {0}; // Note: This ratio calculation might break if 56 bits of precision is used on a 64 bit machine, or 24 bits on a 32 bit machine
        return Box::from(format!("\x1b[38;2;{r};{g};{b}m", r=255-ratio, g=ratio, b=0)) // Returns 24 bit color
    }
}

pub struct BlueScheme {}
impl LoadingColorScheme for BlueScheme {
    fn get_char_block_color(&self, _l_elem: &LoadingElement) -> Box<str> {
        Box::from(format!("\x1b[38;2;{r};{g};{b}m", r=0, g=0, b=255)) // Returns 24 bit color
    }
}

// TODO: This might work more effectively as a macro
pub fn set_terminal_pos(pos: C2U16) {
    print!("\x1B[{line};{column}H", column = pos.x as usize, line = pos.y as usize);
}

// Note: Does not flush stdout
// Prints Unicode U+2500 '─'
pub fn print_splitter_line(terminal_size: &C2U16, offset_height: u16) {
    set_terminal_pos(C2U16 { x: 0, y: offset_height }); // Set proper positioning
    print!("{end:\u{2500}>times$}", end="", times=terminal_size.x as usize); // Print 
}


