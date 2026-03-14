#![allow(non_snake_case)]

use std::env;

struct Image {
    pixels: Vec<i16>,
    width: i32,
    height: i32
}

fn main() {
    println!("Hello, world!");

    //Drop first value since it's the file being called
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);
    if args.len() == 0 {
        print!("No path defined!");
    }
}