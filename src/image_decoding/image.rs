#[derive(Debug, Clone, Copy)]
pub struct colourRGBA {
    pub R: u8,
    pub G: u8,
    pub B: u8,
    pub A: u8,
}

#[derive(Debug)]
pub struct Image {
    pub pixels: Vec<colourRGBA>,
    pub width: u32,
    pub height: u32,
    pub depth: u8,
    pub colourType: u8,
    pub colourSpace: u8, //No idea if I'll actually need this we'll see! ~~~~~~~~~~~~~~
    //Ignore compressionfilter and interlace
    pub colourPalette: Vec<colourRGBA>,
}