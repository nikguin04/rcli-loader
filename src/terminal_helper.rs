use terminal_size::{Width, Height, terminal_size};


pub fn get_terminal_size() -> Result<[u16; 2], &'static str> {
    
    if let Some((Width(w), Height(h))) = terminal_size() {
        println!("Width: {}, Height: {}", w, h);
        return Ok([w,h]);
    } else {
        println!("Unable to get terminal size");
        return Err("Unable to get terminal size");
    }

}