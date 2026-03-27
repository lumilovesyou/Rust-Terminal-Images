#[derive(Debug, Clone, Copy)]
pub struct colourRGBA {
    pub R: u8,
    pub G: u8,
    pub B: u8,
    pub A: u8,
}

impl Default for colourRGBA {
    fn default() -> Self {
        colourRGBA { R: 0, G: 0, B: 0, A: 255 }
    }
}

impl colourRGBA {
    pub fn map<F: Fn(u8) -> u8>(&self, f: F) -> colourRGBA {
        colourRGBA {
            R: f(self.R),
            G: f(self.G),
            B: f(self.B),
            A: self.A,
        }
    }

    pub fn greyscale(&self, f: u8) -> colourRGBA {
        colourRGBA {
            R: f,
            G: f,
            B: f,
            A: self.A,
        }
    }
}

#[derive(Debug)]
pub struct Image {
    pub pixels: Vec<colourRGBA>,
    pub width: u32,
    pub height: u32,
    pub depth: u8,
    pub gamma: f32,
    pub colourType: u8,
    pub colourSpace: u8, //No idea if I'll actually need this we'll see! ~~~~~~~~~~~~~~
    //Ignore compressionfilter and interlace
    pub colourPalette: Vec<colourRGBA>,
}

impl Default for Image {
    fn default() -> Self {
        Image { pixels: vec![], width: 0, height: 0, depth: 0, gamma: -1.0, colourType: 0, colourSpace: 0, colourPalette: vec![] }
    }
}