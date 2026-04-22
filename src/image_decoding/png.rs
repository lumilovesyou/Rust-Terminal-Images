use flate2::read::ZlibDecoder; //Ewww gross a libraryyyyy >m<
use std::io::Read;
use std::num::Wrapping;
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

fn applyGamma(image: &Image, value: u8) -> u8 {
    if image.gamma != -1.0 {
        return ((value as f32 / 255.0).powf(image.gamma) * 255.0) as u8;
    }
    return value;
}

fn paethFilter(a: u8, b: u8, c: u8) -> u8 {
    let a = a as i32;
    let b = b as i32;
    let c = c as i32;
    let p = a + b - c;
    let pa = (p - a).abs();
    let pb = (p - b).abs();
    let pc = (p - c).abs();
    if pa <= pb && pa <= pc { a as u8 }
    else if pb <= pc { b as u8 }
    else { c as u8 }
}


fn decodeRows(image: &Image, decompressedBytes: &Vec<u8>, i: &mut usize, width: u32, height: u32) -> Vec<colourRGBA> {
    let mut pixels: Vec<colourRGBA> = vec![];

    for _ in 0..height {
        let lineFilter = readU8(i, &decompressedBytes);

        println!("Line filter: {}", lineFilter);
    
        match image.colourType {
            0 => { //Greyscale
                if image.depth == 8 {
                    for j in 0..width {
                        let mut value = readU8(i, decompressedBytes);
                        let left      = if j > 0 { pixels[pixels.len() - 1].R } else { 0 };
                        let above     = if pixels.len() >= width as usize { pixels[pixels.len() - width as usize].R } else { 0 };
                        let aboveleft = if j > 0 && pixels.len() > width as usize { pixels[pixels.len() - width as usize - 1].R } else { 0 };
                        value = match lineFilter {
                            1 => (Wrapping(value) + Wrapping(left)).0,
                            2 => (Wrapping(value) + Wrapping(above)).0,
                            3 => (Wrapping(value) + Wrapping(((left as u16 + above as u16) / 2) as u8)).0,
                            4 => (Wrapping(value) + Wrapping(paethFilter(left, above, aboveleft))).0,
                            _ => value,
                        };
                        pixels.push(colourRGBA::default().greyscale(value));
                    }
                } else {
                    for _ in 0..((width * image.depth as u32) + 7) / 8 {
                        let byte = readU8(i, decompressedBytes);

                        for j in (0..8u32).step_by(image.depth as usize) {
                            if j < width * image.depth as u32 {
                                let shiftBy = 8 - image.depth - j as u8;
                                let greyscaleValue = (byte >> shiftBy) & ((1 << image.depth) - 1);
                                let scaled = (greyscaleValue as u32 * 255 / ((1u32 << image.depth) - 1)) as u8;
                                pixels.push(colourRGBA::default().greyscale(scaled));
                            }
                        }
                    }
                }
            },
            3 => { //Indexed/Palette
                if image.depth == 8 {
                    for _ in 0..width {
                        let byte = readU8(i, decompressedBytes);
                        pixels.push(image.colourPalette[byte as usize]);
                    }
                } else {
                    for _ in 0..((width * image.depth as u32) + 7) / 8 {
                        let byte = readU8(i, decompressedBytes);

                        for j in (0..8u32).step_by(image.depth as usize) {
                            if j < width * image.depth as u32 {
                                let shiftBy = 8 - image.depth - j as u8;
                                let colourIndex = (byte >> shiftBy) & ((1 << image.depth) - 1);
                                pixels.push(image.colourPalette[colourIndex as usize]);
                            }
                        }
                    }
                }
            },
            2 | 6 => { //RGB / RGBA
                for j in 0..width {
                    let mut colour = colourRGBA { R: readU8(i, &decompressedBytes), G: readU8(i, &decompressedBytes), B: readU8(i, &decompressedBytes), A: if image.colourType == 6 { readU8(i, &decompressedBytes) } else { 255 } };
                    
                    match lineFilter {
                        1 => {
                            if j != 0 {
                                //Turn this into an impl because it's ugly and I pretend code in image.rs doesn't exist :{ ~~~~~~~~~~~~~~
                                let previous = pixels[pixels.len() - 1];
                                colour.R = (Wrapping(colour.R) + Wrapping(previous.R)).0;
                                colour.G = (Wrapping(colour.G) + Wrapping(previous.G)).0;
                                colour.B = (Wrapping(colour.B) + Wrapping(previous.B)).0;
                                colour.A = (Wrapping(colour.A) + Wrapping(previous.A)).0;
                            }
                        },
                        2 => { //Up
                            let above = pixels[pixels.len() - width as usize];
                            colour.R = (Wrapping(colour.R) + Wrapping(above.R)).0;
                            colour.G = (Wrapping(colour.G) + Wrapping(above.G)).0;
                            colour.B = (Wrapping(colour.B) + Wrapping(above.B)).0;
                            colour.A = (Wrapping(colour.A) + Wrapping(above.A)).0;
                        },
                        3 => { //Average
                            let left  = if j > 0 { pixels[pixels.len() - 1] } else { colourRGBA::default() };
                            let above = if pixels.len() >= width as usize { pixels[pixels.len() - width as usize] } else { colourRGBA::default() };
                            colour.R = (Wrapping(colour.R) + Wrapping(((left.R as u16 + above.R as u16) / 2) as u8)).0;
                            colour.G = (Wrapping(colour.G) + Wrapping(((left.G as u16 + above.G as u16) / 2) as u8)).0;
                            colour.B = (Wrapping(colour.B) + Wrapping(((left.B as u16 + above.B as u16) / 2) as u8)).0;
                            colour.A = (Wrapping(colour.A) + Wrapping(((left.A as u16 + above.A as u16) / 2) as u8)).0;
                        },
                        4 => { //Paeth
                            let left  = if j > 0 { pixels[pixels.len() - 1] } else { colourRGBA::default() };
                            let above = if pixels.len() >= width as usize { pixels[pixels.len() - width as usize] } else { colourRGBA::default() };
                            let aboveleft = if j > 0 && pixels.len() >= width as usize { pixels[pixels.len() - width as usize - 1] } else { colourRGBA::default() };
                            colour.R = (Wrapping(colour.R) + Wrapping(paethFilter(left.R, above.R, aboveleft.R))).0;
                            colour.G = (Wrapping(colour.G) + Wrapping(paethFilter(left.G, above.G, aboveleft.G))).0;
                            colour.B = (Wrapping(colour.B) + Wrapping(paethFilter(left.B, above.B, aboveleft.B))).0;
                            colour.A = (Wrapping(colour.A) + Wrapping(paethFilter(left.A, above.A, aboveleft.A))).0;
                        }
                        _ => {},
                    }

                    colour = colour.map(|v| applyGamma(&image, v));
                    pixels.push(colour);
                }
            },
            _ => {},
        }
    }

    return pixels;
}

pub fn readPNG(imageBytes: &Vec<u8>) -> Image {
    let mut image = Image::default();

    let mut i = 8; //Starts at eight to skip magic bytes
    let mut chunkLength;
    let mut chunkType;
    let mut idatChunks: Vec<u8> = vec![];

    while i < imageBytes.len() {
        //Get length of chunk
        chunkLength = readU32(&mut i, &imageBytes) as usize;

        //Get chunk type
        chunkType = readU32(&mut i, &imageBytes);

        println!("Chunk type: {}\n{}{}{}{}", chunkType, imageBytes[i - 4] as char, imageBytes[i - 3] as char, imageBytes[i - 2] as char, imageBytes[i - 1] as char);

        match chunkType {
            //Maybe use something like b"IHDR" to make this more readable? ~~~~~~~~~~~~~~
            1229472850 => { //IHDR
                image.width = readU32(&mut i, &imageBytes);
                image.height = readU32(&mut i, &imageBytes);
                image.depth = readU8(&mut i, &imageBytes);
                image.colourType = readU8(&mut i, &imageBytes);
                i += 2; //Skip unneeded fields
                image.interlace = readU8(&mut i, &imageBytes);
            },
            1732332865 => { //gAMA
                image.gamma = readU32(&mut i, &imageBytes) as f32 / 100000.0;
            },
            1934772034 => { //sRGB
                image.colourSpace = readU8(&mut i, &imageBytes);
            },
            1347179589 => { //PLTE
                for _ in (0..chunkLength).step_by(3) {
                    //Convert to the colour format
                    let bytes = readVec(&mut i, 3, &imageBytes);
                    image.colourPalette.push(colourRGBA { R: bytes[0] as u8, G: bytes[1] as u8, B: bytes[2] as u8, A: 255 });
                }
            },
            1951551059 => { //tRNS
                for j in 0..chunkLength {
                    //Add alpha to colour format
                    let byte = readVec(&mut i, 1, &imageBytes);
                    image.colourPalette[j as usize].A = byte[0] as u8;
                }
            },
            1229209940 => { //IDAT
                //Adds all the pixel values to a list to later be processed
                idatChunks.extend(readVec(&mut i, chunkLength, &imageBytes));
            },
            _ => {
                //Skip unknown/unnecessary chunks
                //IEND - Skipped because no data is contained
                //tEXt, zTXt, iTXt, tIME, pHYs - Skipped because they're just metadata
                //bKGD - Skipped because I already implement a background colour system
                //iCCP - Skipped because I don't want to deal with colour profiles
                i += chunkLength;
            }
        }
        i += 4; //Skip checksum (criminal)
    }
    //Decompresses the bytes. I am *not* writing a zlib decompressor by hand today
    let mut zlibDecoder = ZlibDecoder::new(&idatChunks[..]);
    let mut decompressedBytes: Vec<u8> = vec![];
    zlibDecoder.read_to_end(&mut decompressedBytes).unwrap();

    println!("Image size: {}x{}", image.width, image.height);
    println!("Colour type: {}", image.colourType);
    println!("Depth: {}", image.depth);


    let mut i = 0;
    if image.interlace == 1 {
        //blehhhhh
        const ADAM7_X_START: [u32; 7] = [0, 4, 0, 2, 0, 1, 0];
        const ADAM7_Y_START: [u32; 7] = [0, 0, 4, 0, 2, 0, 1];
        const ADAM7_X_STEP:  [u32; 7] = [8, 8, 4, 4, 2, 2, 1];
        const ADAM7_Y_STEP:  [u32; 7] = [8, 8, 8, 4, 4, 2, 2];

        image.pixels.resize(
            (image.width * image.height) as usize,
            colourRGBA::default()
        );

        for pass in 0..7 {
            let xStart = ADAM7_X_START[pass];
            let yStart = ADAM7_Y_START[pass];
            let xStep  = ADAM7_X_STEP[pass];
            let yStep  = ADAM7_Y_STEP[pass];

            let passWidth  = (image.width  + xStep - 1 - xStart) / xStep;
            let passHeight = (image.height + yStep - 1 - yStart) / yStep;
            
            if passWidth == 0 || passHeight == 0 { continue; }
            let passPixels = decodeRows(&image, &decompressedBytes, &mut i, passWidth, passHeight);
            for row in 0..passHeight {
                for col in 0..passWidth {
                    let endX = xStart + col * xStep;
                    let endY = yStart + row * yStep;
                    let endIdx = (endY * image.width + endX) as usize;
                    let srcIdx  = (row * passWidth + col) as usize;
                    image.pixels[endIdx] = passPixels[srcIdx];
                }
            }
        }
    } else {
        image.pixels = decodeRows(&image, &decompressedBytes, &mut i, image.width, image.height)
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
        iCCP Embedded ICC profile ✓
    Textual information [✓]
        tEXt Textual data ✓
        zTXt Compressed textual data ✓
        iTXt International textual data  ✓
    Miscellaneous information [ ]
        bKGD Background color ✓
        pHYs Physical pixel dimensions ✓
        sBIT Significant bits
        sPLT Suggested palette
        hIST Palette histogram
        tIME Image last-modification time ✓

Other to-do:
Replace file recognizer library with just magic byte checks
Find a better way to represent chunk ids in code //On that note I'm dumb and apparently each chunk id is actually the ascii character representation, so there you go
*/