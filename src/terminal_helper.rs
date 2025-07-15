use terminal_size::{Width, Height, terminal_size};

// Coordinate 2d of u16
pub struct C2U16 {
    pub x: u16, pub y: u16
} 

pub fn get_terminal_size() -> Result<C2U16, &'static str> {
    
    if let Some((Width(w), Height(h))) = terminal_size() {
        //println!("Width: {}, Height: {}", w, h);
        return Ok(C2U16 { x: w, y: h });
    } else {
        //println!("Unable to get terminal size");
        return Err("Unable to get terminal size");
    }

}