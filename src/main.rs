#![allow(non_snake_case)]

use std::{env, fs::read, ops::Deref, process::exit};
use file_type::FileType;
use std::path::Path;


#[derive(Debug)]
struct colourRGB {
    R: u8,
    G: u8,
    B: u8,
}

#[derive(Debug)]
struct Image {
    pixels: Vec<u8>,
    width: u32,
    height: u32,
    depth: u8,
    colourSpace: u8, //No idea if I'll actually need this we'll see! ~~~~~~~~~~~~~~
    //Ignore colourType, compressionfilter, interlace
    colourPalette: Vec<colourRGB>,
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

//There's probably a better way to do this than making two almost identical functions
fn readSection(start: &mut usize, values: usize, list: &Vec<u8>) -> Vec<u32> {
    let mut total = vec![];
    for i in 0..values {
        total.push(list[*start + i] as u32);
    }
    *start += values;
    return total;
}

fn readSectionMult(start: &mut usize, values: usize, list: &Vec<u8>) -> u32 {
    let mut total: u32 = 0;
    for i in 0..values {
        total += (list[*start + i] as u32) << (8 * (3 - i));
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
        depth: 0,
        colourSpace: 0,
        colourPalette: vec![],
    };

    let mut i = 0;
    let mut chunkLength;
    let mut chunkType;
    while i < imageBytes.len() {
        //Get length of chunk
        chunkLength = readSectionMult(&mut i, 4, &imageBytes);

        //Get chunk type
        chunkType = readSectionMult(&mut i, 4, &imageBytes);
        match chunkType {
            1229472850 => { //IHDR
                image.width = readSectionMult(&mut i, 4, &imageBytes);
                image.height = readSectionMult(&mut i, 4, &imageBytes);
                image.depth = readSectionMult(&mut i, 1, &imageBytes) as u8;
                i += 8; //Skip unnecessary fields & checksum (criminal)
            },
            1934772034 => { //sRGB
                image.colourSpace = readSectionMult(&mut i, 1, &imageBytes) as u8;
                i += 4; //Skip checksum
            },
            1883789683 => { //pHYs
                //I don't think we need this...? I'll just skip it. Checksum included.
                i += 13;
            },
            1347179589 => { //PLTE
                for j in (0..chunkLength).step_by(3) {
                    //Convert to the colour format
                    let bytes = readSection(&mut i, 3, &imageBytes);
                    image.colourPalette.push(colourRGB { R: bytes[0] as u8, G: bytes[1] as u8, B: bytes[2] as u8 });
                }
            },
            _ => {
                //Skip unknown chunks
                i += chunkLength as usize + 4;
            }
        }
    }
}
