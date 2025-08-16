use terminal_size::{Width, Height, terminal_size};

// Coordinate 2d of u16
pub struct V2Usz {
    pub x: usize, pub y: usize
} 

pub fn get_terminal_size() -> V2Usz { //Result<V2Usz, &'static str> { - Temporarily disabled result returning to default a size instead
    if let Some((Width(w), Height(h))) = terminal_size() {
        return V2Usz { x: w as usize, y: h as usize};
    } else {
        //return Err("Unable to get terminal size");
        return V2Usz {x:70, y:20};
    }
}

pub fn get_line_count(count_str: &String) -> usize { // Line count is equal to both the newline characters, and the amount of wrapping lines (one "line" can wrap multiple times)
    let ts: V2Usz = get_terminal_size();
    let mut count: usize = 0;
    count_str.lines().for_each(|line: &str| {
        count += 1 + line.len() / ts.x as usize;
    });
    return count;
}