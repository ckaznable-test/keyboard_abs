use std::{sync::Weak, time::{SystemTime, UNIX_EPOCH, Instant, Duration}, thread};

use bbbbbb::keyboard::Keyboard;
use bevy::{utils::HashMap, prelude::KeyCode};

fn main() {
    let start = Instant::now();
    let my_instance = Keyboard::new();
    let elapsed = start.elapsed();
    println!("Elapsed: {}ns", elapsed.as_nanos());

    let size = std::mem::size_of_val(&my_instance);
    println!("Size of MyStruct instance: {} bytes", size);
}

