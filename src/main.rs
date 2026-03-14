#![allow(non_snake_case)]

use std::{env, process::exit, fs};
use std::path::Path;

struct Image {
    pixels: Vec<i16>,
    width: i32,
    height: i32
}

fn main() {
    let mut path = "";

    //Drop first value since it's the file being called
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);
    if args.len() == 0 {
        close("No path provided!")
    } else {
        path = args[args.len() - 1].as_str();
    }

    if !Path::new(path).exists() {
        close("Path is invalid!")
    }
}

fn close(reason: &str) {
    print!("{}", reason);
    exit(0)
}