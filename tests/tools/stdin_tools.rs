use rcli_loader::tools::stdin_tools::{relative_position_in_bounds};



#[test]
fn relative_position_in_bounds_negative() {
    assert!(relative_position_in_bounds(100, 50, -1));
    assert!(!relative_position_in_bounds(80, 0, -1));
    assert!(relative_position_in_bounds(100, 1, -1));
    assert!(!relative_position_in_bounds(100, 10, -11));
    assert!(relative_position_in_bounds(100, 10, -10));
}

#[test]
fn relative_position_in_bounds_positive() {
    assert!(relative_position_in_bounds(100, 99, 1));
    assert!(!relative_position_in_bounds(80, 9, 75));
    assert!(relative_position_in_bounds(100, 99, 1));
    assert!(!relative_position_in_bounds(100, 99, 2));
    assert!(relative_position_in_bounds(100, 10, 10));
}

#[test]
fn relative_position_in_bounds_zero() {
    assert!(relative_position_in_bounds(0, 0, 0));
    assert!(relative_position_in_bounds(3, 3, 0));
}
