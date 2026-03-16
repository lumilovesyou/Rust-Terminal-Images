use super::image_decoding::image::colourRGBA;

pub fn makeColoured(text: &str, colour: colourRGBA) -> String {
    let A = colour.A as f32 / 255.0;
    let mut R = colour.R;
    let mut G = colour.G;
    let mut B = colour.B;
    
    R = (R as f32 * A * (0.0 + A)) as u8;
    G = (G as f32 * A * (0.0 + A)) as u8;
    B = (B as f32 * A * (0.0 + A)) as u8;

    return format!("\x1b[38;2;{};{};{}m{}\x1b[0m", R, G, B, text)
}

pub fn print(text: &str, colour: colourRGBA) {
    print!("\x1b[38;2;{};{};{}m{}", colour.R, colour.G, colour.B, text);
}


pub fn pixel(colour: colourRGBA) {
    print!("\x1b[48;2;{};{};{}m  \x1b[0m", colour.R, colour.G, colour.B);
}