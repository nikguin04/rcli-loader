

pub fn relative_position_in_bounds (region: usize, position: usize, difference: i32) -> bool {
    return
        (difference < 0 && position >= difference.abs() as usize)  // Difference is negative, but does not exceed 0 to negative
        || (difference >= 0 && position+(difference as usize) <= region) 
}