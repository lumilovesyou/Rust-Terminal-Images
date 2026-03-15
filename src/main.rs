#![allow(non_snake_case)]

use std::collections::btree_map::Values;
use std::{env, fmt, fs::read, process::exit, vec};
use flate2::read::ZlibDecoder; //Ewww gross a libraryyyyy >m<
use file_type::FileType;
use std::path::Path;
use std::io::Read;


#[derive(Debug, Clone, Copy)]
struct colourRGBA {
    R: u8,
    G: u8,
    B: u8,
    A: u8,
}

#[derive(Debug)]
struct Image {
    pixels: Vec<colourRGBA>,
    width: u32,
    height: u32,
    depth: u8,
    colourType: u8,
    colourSpace: u8, //No idea if I'll actually need this we'll see! ~~~~~~~~~~~~~~
    //Ignore compressionfilter and interlace
    colourPalette: Vec<colourRGBA>,
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
fn readSection(start: &mut usize, values: usize, list: &Vec<u8>) -> Vec<u8> {
    let mut total = vec![];
    for i in 0..values {
        total.push(list[*start + i]);
    }
    *start += values;
    return total;
}

fn readSectionMult(start: &mut usize, values: usize, list: &Vec<u8>) -> u32 {
    let mut total: u32 = 0;
    for i in 0..values {
        total += (list[*start + i] as u32) << (8 * ((values - 1) - i));
    }
    *start += values;
    return total;
}

fn readSectionAdd(start: &mut usize, values: usize, list: &Vec<u8>) -> u32 {
    let mut total: u32 = 0;
    for i in 0..values {
        total += list[*start + i] as u32;
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
        colourType: 0,
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
        //println!("{}", chunkType); //~~~~~~~~~~~~~~
        match chunkType {
            1229472850 => { //IHDR
                image.width = readSectionMult(&mut i, 4, &imageBytes);
                image.height = readSectionMult(&mut i, 4, &imageBytes);
                image.depth = readSectionMult(&mut i, 1, &imageBytes) as u8;
                image.colourType = readSectionMult(&mut i, 1, &imageBytes) as u8;
                println!("Size: {} x {}", image.width, image.height);
                println!("Colourtype: {}\nDepth: {}", image.colourType, image.depth);
                i += 7; //Skip unnecessary fields & checksum (criminal)
            },
            1934772034 => { //sRGB
                image.colourSpace = readSectionMult(&mut i, 1, &imageBytes) as u8;
                println!("Colourspace: {}", image.colourSpace);
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
                    image.colourPalette.push(colourRGBA { R: bytes[0] as u8, G: bytes[1] as u8, B: bytes[2] as u8, A: 255 });
                }
                i += 4;
            },
            1951551059 => { //tRNS
                for j in 0..chunkLength {
                    //Add alpha to colour format
                    let byte = readSection(&mut i, 1, &imageBytes);
                    image.colourPalette[j as usize].A = byte[0] as u8;
                }
                i += 4;
            },
            1229209940 => { //IDAT
                //Decompresses the bytes. I am *not* writing a zlib decompressor by hand today
                let bytesToDecode = readSection(&mut i, chunkLength as usize, &imageBytes);
                let mut zlibDecoder = ZlibDecoder::new(&bytesToDecode[..]);
                let mut decompressedBytes: Vec<u8> = vec![];
                zlibDecoder.read_to_end(&mut decompressedBytes).unwrap();
                println!("{:?}", decompressedBytes);

                let mut j = 0;
                for k in 0..image.height {
                    let lineFilter = readSectionAdd(&mut j, 1, &decompressedBytes) as u8;
                    
                    for l in 0..((image.width * image.depth as u32) + 7) / 8 {
                        let byte = readSectionAdd(&mut j, 1, &decompressedBytes) as u8;

                        for m in (0..8).step_by(image.depth as usize) {
                            if m < image.width * image.depth as u32 {
                                let shiftBy = 8 - image.depth - m as u8;
                                let colourIndex = (byte >> shiftBy) & ((1 << image.depth) - 1); //Shift magic @w@
                                image.pixels.push(image.colourPalette[colourIndex as usize])
                            }
                        }
                    }
                }

                println!("{:?}", image.pixels);

                i += 4;
            },
            _ => {
                //Skip unknown chunks
                i += chunkLength as usize + 4;
            }
        }
    }
}
