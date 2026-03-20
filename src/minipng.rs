#[derive(Debug)]
#[derive(PartialEq)]
pub enum PixelType {
    BW,
    GRAYSCALE,
    PALETTE,
    TRUECOLOR,
}

#[derive(Debug)]
pub struct Minipng {
    pub comments: Vec<String>,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub pixel_type: Option<PixelType>,
    pub palette: Option<Vec<(u8, u8, u8)>>,
    pub data: Vec<u8>,
}

