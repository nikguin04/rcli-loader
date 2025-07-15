use std::char;

fn main() {
    //println!("\u{258B}")
    let a: char = 'b';
    println!("{fc:0>len$}", fc=a, len=4);
}