use minifb::{Key, Window, WindowOptions};

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
    
    pub fn is_bw(&self) -> bool {
        for pixel in &self.pixels {
            match pixel.color() {
                (0, 0, 0) | (255, 255, 255) => {},
                _ => return false,
            }
        }
        return true;
    }
    
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
    
    pub fn display_color(&self, pixel_size: u32) {
        let mut buffer: Vec<u32> = Vec::new();
        
        for i in 0..(self.size.1 * pixel_size) {
            for j in 0..self.size.0 {
                let (r, g, b) = self.pixels[((i / pixel_size) * self.size.0 + j) as usize].color();
                for _ in 0..pixel_size {
                    buffer.push(((r as u32) << 16) | ((g as u32) << 8) | (b as u32));
                }
            }
        }
        
        println!("opening window...");
        
        let mut window = Window::new(
            "Image",
            (self.size.0 * pixel_size) as usize,
            (self.size.1 * pixel_size) as usize,
            WindowOptions::default(),
        ).unwrap();
        
        window.set_target_fps(60);
        
        window.update_with_buffer(&buffer, (self.size.0 * pixel_size) as usize, (self.size.1 * pixel_size) as usize).unwrap();
        
        while window.is_open() && !window.is_key_down(Key::Escape) {
            window.update();
        }
    }
}