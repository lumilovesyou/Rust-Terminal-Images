#![allow(non_snake_case)]

use std::{env, fs::read, ops::Deref, process::exit};
use file_type::FileType;
use std::path::Path;

struct Image {
    pixels: Vec<u8>,
    width: i32,
    height: i32
}

fn main() {
    let mut path = "";

    //Get input arguments
    let mut args: Vec<String> = env::args().collect();
    args.remove(0); //Drop first value since it's the file being called

    //Check if a path is provided
    if args.len() == 0 {
        close("No path provided!")
    } else {
        path = args[args.len() - 1].as_str();
    }

    //Check whether value exists
    if !Path::new(path).exists() {
        close("Path is invalid!")
    }

    //Read file bytes
    let fileBytes = read(path).ok().unwrap_or(vec![]);

    //Checks for empty file
    if fileBytes.len() == 0 {
        close("File is empty!");
    }

    //Get file type ~~~~~~~~~~~~~~ Replace with my own code
    let fileType = FileType::from_bytes(&fileBytes).media_types().first().copied().unwrap();

    //Matches supported file types to read functions
    match fileType {
        "image/png" => print!("{:?}", readPNG(&fileBytes)), //89, 50, 4E, 47, 0D, 0A, 1A, 0A
        _ => close("Filetype isn't PNG!")
    }
}

fn close(reason: &str) {
    //Print reason before closing
    print!("{}", reason);
    exit(0)
}

fn readPNG(bytes: &Vec<u8>) {
    //Removes PNG magic bytes
    let imageBytes = bytes.clone().split_off(8);
    print!("{:?}", imageBytes);
}
