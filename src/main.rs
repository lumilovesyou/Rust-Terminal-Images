#![allow(non_snake_case)]

use std::{env, fs::read, process::exit, vec};
use file_type::FileType;
use std::path::Path;

mod image_decoding;
use image_decoding::{png::readPNG, image};

use crate::image_decoding::image::Image;

mod true_colour;
use true_colour::print;

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
    let mut image = Image {
        pixels: vec![],
        width: 0,
        height: 0,
        depth: 0,
        colourType: 0,
        colourSpace: 0,
        colourPalette: vec![],
    };
    match fileType {
        "image/png" => image = readPNG(&fileBytes), //89, 50, 4E, 47, 0D, 0A, 1A, 0A
        //Might add JPEG/JPG and WEBP. Maybe. Probably not.
        _ => close("Filetype isn't PNG!")
    }

    if image.pixels.len() == 0 {
        close("Failed to decode image!");
    }

    for i in image.pixels {
        print("██", i);
    }
}

fn close(reason: &str) {
    //Print reason before closing
    print!("{}", reason);
    exit(0)
}


