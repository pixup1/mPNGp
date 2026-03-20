#[derive(Debug)]
pub struct Pixel {
    color: (u8, u8, u8),
}

impl Pixel {
    pub fn new(color: (u8, u8, u8)) -> Self {
        Self { color }
    }
    
    pub fn color(&self) -> (u8, u8, u8) {
        self.color
    }
}

#[derive(Debug)]
pub struct Image {
    size: (u32, u32),
    pixels: Vec<Pixel>,
}

// This struct represents an image, independently of the file format it was parsed from
impl Image {
    pub fn new(size: (u32, u32), pixels: Vec<Pixel>) -> Self {
        if size.0 == 0 || size.1 == 0 {
            panic!("Image width and height must be greater than 0");
        }
        
        if pixels.len() as u32 != size.0 * size.1 {
            panic!("Number of pixels must be equal to width * height");
        }
        
        Self {
            size,
            pixels,
        }
    }
    
    // pub fn size(&self) -> (u32, u32) {
    //     self.size
    // }

    // pub fn pixels(&self) -> &Vec<Pixel> {
    //     &self.pixels
    // }
    
    pub fn print_bw(&self) {
        let mut bw = true;
        
        for y in 0..self.size.1 {
            for x in 0..self.size.0 {
                let pixel = &self.pixels[(y * self.size.0 + x) as usize];
                match pixel.color() {
                    (0, 0, 0) => print!("X"),
                    (255, 255, 255) => print!(" "),
                    _ => {
                        bw = false;
                        print!("ඞ");
                    },
                }
            }
            println!();
        }
        
        if !bw {
            println!();
            println!("Warning: Image.print_bw() is meant to print BW images, this one is not.");
        }
    }
}