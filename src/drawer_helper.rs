use crate::loading_element::LoadingElement;

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



