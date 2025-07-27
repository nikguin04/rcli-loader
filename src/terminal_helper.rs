use terminal_size::{Width, Height, terminal_size};

// Coordinate 2d of u16
pub struct C2U16 {
    pub x: u16, pub y: u16
} 

pub fn get_terminal_size() -> C2U16 { //Result<C2U16, &'static str> { - Temporarily disabled result returning to default a size instead
    
    if let Some((Width(w), Height(h))) = terminal_size() {
        return C2U16 { x: w, y: h };
    } else {
        //return Err("Unable to get terminal size");
        return C2U16 {x:70, y:20};
    }

}