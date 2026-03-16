use super::image_decoding::image::colourRGBA;

pub fn print(text: &str, color: colourRGBA) {
    print!("\x1b[38;2;{};{};{}m{}", color.R, color.G, color.B, text);
}


pub fn pixel(color: colourRGBA) {
    print!("\x1b[48;2;{};{};{}m  \x1b[0m", color.R, color.G, color.B);
}