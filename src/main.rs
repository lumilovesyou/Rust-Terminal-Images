#![allow(non_snake_case)]

use std::{env, fs::read, ops::Deref, process::exit};
use file_type::FileType;
use std::path::Path;

#[derive(Debug)]
struct Image {
    pixels: Vec<u8>,
    width: u32,
    height: u32,
    depth: u8
    //Ignore colorType, compressionfilter, interlace
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

fn readSection(start: &mut usize, values: usize, list: &Vec<u8>) -> u32 {
    let mut total: u32 = 0;
    for i in 0..values {
        total = total + list[*start + i] as u32;
    }
    *start += values;
    return total;
}

fn readPNG(bytes: &Vec<u8>) {
    //Removes PNG magic bytes
    let imageBytes = bytes.clone().split_off(8);
    
    let mut image = Image {
        pixels: vec![],
        width: 0,
        height: 0,
        depth: 0
    };

    let mut i = 0;
    let mut chunkLength = 0;
    let mut chunkType = 0;
    while i < imageBytes.len() {
        //Get length of chunk
        chunkLength = readSection(&mut i, 4, &imageBytes);

        //Get chunk type
        chunkType = readSection(&mut i, 4, &imageBytes);
        println!("{}", chunkType);
        match chunkType {
            295 => { //IHDR
                image.width = readSection(&mut i, 4, &imageBytes);
                image.height = readSection(&mut i, 4, &imageBytes);
                image.depth = readSection(&mut i, 1, &imageBytes) as u8;
                i += 4;
                print!("{:?} x {:?}", image.width, image.height);
            },
            _ => {
                //Skip unknown chunks
                i += chunkLength as usize + 4;
            }
        }
    }
}
