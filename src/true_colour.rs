use super::image_decoding::image::colourRGBA;

pub fn makeColoured(text: &str, colour: colourRGBA, alpha: colourRGBA) -> String {
    let A = colour.A as f32 / 255.0;
    let mut R = colour.R;
    let mut G = colour.G;
    let mut B = colour.B;
    
    R = (R as f32 * A + alpha.R as f32 * (1.0 - A)) as u8;
    G = (G as f32 * A + alpha.G as f32 * (1.0 - A)) as u8;
    B = (B as f32 * A + alpha.B as f32 * (1.0 - A)) as u8;

    return format!("\x1b[38;2;{};{};{}m{}\x1b[0m", R, G, B, text)
}

pub fn hexToRGB(hex: String) -> colourRGBA {
    let mut hexCode = hex;
    let mut colour = colourRGBA::default();

    //Drop hashtag in hex code
    if hexCode.chars().nth(0).unwrap() == '#' {
        hexCode.remove(0);
    }

    //This is so gross there's gotta be some way to compress this all down some
    if hexCode.len() == 3 {
        let chars: Vec<char> = hexCode.chars().collect();
        colour.R = u8::from_str_radix(chars[0].to_string().as_str(), 16).unwrap() * 17;
        colour.G = u8::from_str_radix(chars[1].to_string().as_str(), 16).unwrap() * 17;
        colour.B = u8::from_str_radix(chars[2].to_string().as_str(), 16).unwrap() * 17;
    } else if hexCode.len() == 6 {
        colour.R = u8::from_str_radix(&hexCode[0..2], 16).unwrap();
        colour.G = u8::from_str_radix(&hexCode[2..4], 16).unwrap();
        colour.B = u8::from_str_radix(&hexCode[4..6], 16).unwrap();
    } else {
        //Return default value if invalid
        return colourRGBA::default();
    }

    return colour;
}