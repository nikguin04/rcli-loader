use std::{io::Read, time::Duration};

use tokio::time::sleep;

use crossterm::terminal::enable_raw_mode;

#[tokio::main]
async fn main() {
    println!("Locking stdin");
    let mut stdin = std::io::stdin().lock();
    println!("Locked");
    enable_raw_mode().unwrap();
    
    for _i in 0.. 100 {
        //println!("Hello");
        let mut buf: [u8; 1024] = [0; 1024];
        stdin.read(&mut buf).unwrap();
        println!("Buffer: {}", std::str::from_utf8(&buf).unwrap());
        sleep(Duration::from_millis(20)).await;
    }
}