#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use std::{env, fs::read, process::exit, vec};
use file_type::FileType;
use std::path::Path;

mod image_decoding;
use image_decoding::png::readPNG;
use image_decoding::image::{Image, colourRGBA};

mod true_colour;

fn printHelp() {
    let helpText: [&str; 3] = [
        "Options:",
        "    -p, --path <PATH>  Path of image file",
        "    -t, --tmcolour <TERMINAL COLOUR>  Colour used for alpha mixing [default: #000000]"];
    for i in 0..helpText.len() {
        println!("{}", helpText[i]);
    }
    exit(0);
}

fn main() {
    let mut path = "";
    let mut termBgColour = colourRGBA::default();

    //Get input arguments
    let mut args: Vec<String> = env::args().collect();
    args.remove(0); //Drop first value since it's the file being called

    //Check if a path is provided
    if args.len() == 0 {
        close("No path provided!")
    } else {
        let mut i = 0;
        while i < args.len() {
            match args[i].as_str() {
                "path" | "--path" | "-p" => if args.len() > i + 1 {
                    i += 1;
                    path = args[i].as_str();
                },
                "tmcolour" | "--tmcolour" | "-t" => if args.len() > i + 1 {
                    i += 1;
                    termBgColour = true_colour::hexToRGB(args[i].clone());
                    println!("{:?}", termBgColour);
                },
                "help" | "--help" | "-h" => printHelp(),
                _ => {
                    if i == args.len() - 1 {
                        path = args[i].as_str();
                    } else {
                        close(format!("Invalid argument: \"{}\"", args[i]).as_str())
                    }
                }
            }

            i += 1;
        }
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
    let mut image = Image::default();
    match fileType {
        "image/png" => image = readPNG(&fileBytes), //89, 50, 4E, 47, 0D, 0A, 1A, 0A
        //Might add JPEG/JPG and WEBP. Maybe. Probably not.
        _ => close("Filetype isn't PNG!")
    }

    if image.pixels.len() == 0 {
        close("Failed to decode image!");
    }

    let mut imageString = String::new();
    for i in 0..image.height {
        for j in 0..image.width {
            imageString = format!("{}{}", imageString, true_colour::makeColoured("██", image.pixels[((i * image.width)+j) as usize], termBgColour))
        }
        imageString = format!("{}\n", imageString);
    }
    println!("{}", imageString);
}

fn close(reason: &str) {
    //Print reason before closing
    print!("{}", reason);
    exit(0)
}


