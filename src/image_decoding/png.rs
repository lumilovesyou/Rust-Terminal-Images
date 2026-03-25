use flate2::read::ZlibDecoder; //Ewww gross a libraryyyyy >m<
use std::io::Read;
use super::image::{Image, colourRGBA};

fn readU8(position: &mut usize, list: &Vec<u8>) -> u8 {
    let num = list[*position];
    *position += 1;
    return num;
}

fn readU32(position: &mut usize, list: &Vec<u8>) -> u32 {
    let num = u32::from_be_bytes(list[*position..*position + 4].try_into().unwrap());
    *position += 4;
    return num;
}

fn readVec(position: &mut usize, values: usize, list: &Vec<u8>) -> Vec<u8> {
    let num = list[*position..*position + values].to_vec();
    *position += values;
    return num;
}

pub fn readPNG(imageBytes: &Vec<u8>) -> Image {
    let mut image = Image {
        pixels: vec![],
        width: 0,
        height: 0,
        depth: 0,
        colourType: 0,
        colourSpace: 0,
        colourPalette: vec![],
    };

    let mut i = 8; //Starts at eight to skip magic bytes
    let mut chunkLength;
    let mut chunkType;
    let mut idatChunks: Vec<u8> = vec![];

    while i < imageBytes.len() {
        //Get length of chunk
        chunkLength = readU32(&mut i, &imageBytes) as usize;

        //Get chunk type
        chunkType = readU32(&mut i, &imageBytes);
        println!("Chunk: {}, Length: {}, i: {}", chunkType, chunkLength, i);
        match chunkType {
            //Maybe use something like b"IHDR" to make this more readable? ~~~~~~~~~~~~~~
            1229472850 => { //IHDR
                image.width = readU32(&mut i, &imageBytes);
                image.height = readU32(&mut i, &imageBytes);
                image.depth = readU8(&mut i, &imageBytes);
                image.colourType = readU8(&mut i, &imageBytes);
                i += 7; //Skip unnecessary fields & checksum (criminal)
            },
            1934772034 => { //sRGB
                image.colourSpace = readU8(&mut i, &imageBytes);
                i += 4; //Skip checksum
            },
            1766015824 => { //iCCP
                //Turns out I don't need this if I don't care about colour accuracy. Yibbeee!!! :333 ~~~~~~~~~~~~~~
                i += 4 + chunkLength;
            },
            1883789683 => { //pHYs
                //I don't think we need this...? I'll just skip it. Checksum included.
                i += 13;
            },
            1347179589 => { //PLTE
                for _ in (0..chunkLength).step_by(3) {
                    //Convert to the colour format
                    let bytes = readVec(&mut i, 3, &imageBytes);
                    image.colourPalette.push(colourRGBA { R: bytes[0] as u8, G: bytes[1] as u8, B: bytes[2] as u8, A: 255 });
                }
                i += 4;
            },
            1951551059 => { //tRNS
                for j in 0..chunkLength {
                    //Add alpha to colour format
                    let byte = readVec(&mut i, 1, &imageBytes);
                    image.colourPalette[j as usize].A = byte[0] as u8;
                }
                i += 4;
            },
            1229209940 => { //IDAT
                //Adds all the pixel values to a list to later be processed
                idatChunks.extend(readVec(&mut i, chunkLength, &imageBytes));
                i += 4;
            },
            _ => {
                //Skip unknown/unnecessary chunks
                //IEND included
                i += chunkLength + 4;
            }
            //Probably a better idea to throw the i+=4 to skip the chunk down here instead of it being repeated so many times ~~~~~~~~~~~~~~
        }
    }

    //Decompresses the bytes. I am *not* writing a zlib decompressor by hand today
    let mut zlibDecoder = ZlibDecoder::new(&idatChunks[..]);
    let mut decompressedBytes: Vec<u8> = vec![];
    zlibDecoder.read_to_end(&mut decompressedBytes).unwrap();

    println!("Let's see if it works!! {:?}", image.colourPalette);

    let mut i = 0;
    for _ in 0..image.height {
        let _lineFilter = readU8(&mut i, &decompressedBytes); //Not needed right now
        
        for _ in 0..((image.width * image.depth as u32) + 7) / 8 {
            let byte = readU8(&mut i, &decompressedBytes);

            for j in (0..8).step_by(image.depth as usize) {
                if j < image.width * image.depth as u32 { //To-do: limit loop length instead of checking each loop ~~~~~~~~~~~~~~
                    let shiftBy = 8 - image.depth - j as u8;
                    let colourIndex = (byte >> shiftBy) & ((1 << image.depth) - 1); //Shift magic @w@
                    image.pixels.push(image.colourPalette[colourIndex as usize])
                }
            }
        }
    }

    return image;
}

/* Chunks to-do:

Critical chunks [✓]
    IHDR Image header ✓
    PLTE Palette ✓
    IDAT Image data ✓
    IEND Image trailer ✓

Ancillary chunks [ ]
    Transparency information [✓]
        tRNS Transparency ✓
    Color space information [ ]
        gAMA Image gamma
        cHRM Primary chromaticities
        sRGB Standard RGB color space ✓
        iCCP Embedded ICC profile 
    Textual information [ ]
        tEXt Textual data
        zTXt Compressed textual data
        iTXt International textual data 
    Miscellaneous information [ ]
        bKGD Background color
        pHYs Physical pixel dimensions ✓
        sBIT Significant bits
        sPLT Suggested palette
        hIST Palette histogram
        tIME Image last-modification time

Other to-do:

Replace file recognizer library with just magic byte checks
Find a better way to represent chunk ids in code //On that note I'm dumb and apparently each chunk id is actually the ascii character representation, so there you go
*/