use std::char;
use std::env;

// fn main() {
//     //println!("\u{258B}")
//     let a: char = 'b';
//     println!("{fc:0>len$}", fc=a, len=4);
// }

fn main() -> std::io::Result<()> {
    let path = env::current_dir()?;
    println!("The current directory is {}", path.display());
    Ok(())
}