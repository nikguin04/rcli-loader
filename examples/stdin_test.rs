use std::{io::Read, time::Duration};

use rcli_loader::drawing::{drawer_helper::set_terminal_pos, terminal_helper::V2Usz};
use tokio::time::sleep;
use crossterm::{
    event::{self, Event, KeyCode},};
use crossterm::terminal::enable_raw_mode;

#[tokio::main]
async fn main() {
    println!("Locking stdin");
    let mut stdin = std::io::stdin().lock();
    println!("Locked");
    enable_raw_mode().unwrap();
    set_terminal_pos(V2Usz { x: 10, y: 15 });
    
    if event::poll(Duration::from_millis(50)).unwrap() {
        if let Event::Key(key_event) = event::read().unwrap() {
            match key_event.code {
                KeyCode::Left => println!("← Left"),
                KeyCode::Right => println!("→ Right"),
                KeyCode::Up => println!("↑ Up"),
                KeyCode::Down => println!("↓ Down"),
                _ => println!("Other: {:?}", key_event),
            }
        }
    }
    for _i in 0.. 100 {
        //println!("Hello");
        let mut buf: [u8; 1024] = [0; 1024];
        stdin.read(&mut buf).unwrap();
        println!("Buffer: {}", std::str::from_utf8(&buf).unwrap());
        sleep(Duration::from_millis(20)).await;
    }
}