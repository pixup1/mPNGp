use std::{io::Error, fs::File, io::Read, path::Path, io::ErrorKind, str::from_utf8};

use crate::minipng::Minipng;
use crate::minipng::PixelType;
use crate::pixels::Image;
use crate::pixels::Pixel;

#[derive(Debug)]
enum BlockType {
    C,
    D,
    H,
    P,
}

pub fn parse_file(path: &Path) -> Result<Image, Error> {
    println!("Input file: {}", path.display());
    
    let mut f = File::open(path)?;
    
    let mut buffer = [0; 8];

    f.read_exact(&mut buffer)?;
    let read_str = match from_utf8(&buffer) {
        Ok(s) => { s },
        Err(e) => { return Err(Error::new(ErrorKind::InvalidData, format!("Could not read magic number: {}", e))) }
    };
    
    if read_str != String::from("Mini-PNG") {
        return Err(Error::new(ErrorKind::InvalidData, format!("Invalid file format: expected Mini-PNG file (got {})", read_str)));
    };
    
    println!("Input is a valid MiniPNG file, parsing...");
    
    let mut png = Minipng {
        comments: Vec::new(),
        width: None,
        height: None,
        pixel_type: None,
        palette: None,
        data: Vec::new(),
    };
    
    let mut eof = false;
    
    while !eof {
        match parse_block(&mut f, &mut png) {
            Ok(_) => {},
            Err(e) => {
                if e.kind() == ErrorKind::UnexpectedEof {
                    println!("Reached end of file.");
                    eof = true;
                } else {
                    return Err(e);
                }
            }
        }
    }
    
    if png.width.is_none() || png.height.is_none() || png.pixel_type.is_none() {
        return Err(Error::new(ErrorKind::InvalidData, "Header block is missing"));
    }
    
    if png.data.is_empty() {
        return Err(Error::new(ErrorKind::InvalidData, "Data block is missing"));
    }
    
    println!("Parsed MiniPNG.");
    // println!("{:#?}", png);
    
    let img = parse_minipng(&png)?;
    
    println!("Parsed image.");
    // println!("{:#?}", img);
    
    println!();
    println!("Width: {}", png.width.unwrap());
    println!("Height: {}", png.height.unwrap());
    println!("Pixel type: {:?}", png.pixel_type.unwrap());
    for comment in png.comments {
        println!("Comment: {}", comment);
    }
    
    return Ok(img);
}

// Parses one block of a Minipng file and updates the given Minipng struct with the data from that block
fn parse_block(f: &mut File, png: &mut Minipng) -> Result<(), Error> {
    let mut buffer = [0; 1];
    f.read_exact(&mut buffer)?;
    let block_type = match buffer[0] as char {
        'C' => {BlockType::C},
        'D' => {BlockType::D},
        'H' => {BlockType::H},
        'P' => {BlockType::P},
        _ => {
            return Err(Error::new(ErrorKind::InvalidData, format!("Unknown block type: {}", buffer[0] as char)));
        }
    };
    
    let mut length_buffer = [0; 4];
    
    f.read_exact(&mut length_buffer)?;
    let length = u32::from_be_bytes(length_buffer);
    println!("Reading block {:?} of length {}...", block_type, length);
    
    let mut data_buffer = vec![0; length as usize];
    
    f.read_exact(&mut data_buffer)?;
    match block_type {
        BlockType::C => {
            let comment = from_utf8(&data_buffer).expect("Could not read comment");
            png.comments.push(String::from(comment));
        },
        BlockType::D => {
            png.data.extend_from_slice(&data_buffer);
        },
        BlockType::H => {
            if length != 9 {
                return Err(Error::new(ErrorKind::InvalidData, "Header block must be 9 bytes long"));
            }
            
            if png.width.is_some() || png.height.is_some() || png.pixel_type.is_some() {
                return Err(Error::new(ErrorKind::InvalidData, "One file cannot contain multiple header blocks"));
            }
            
            png.width = Some(u32::from_be_bytes([data_buffer[0], data_buffer[1], data_buffer[2], data_buffer[3]]));
            png.height = Some(u32::from_be_bytes([data_buffer[4], data_buffer[5], data_buffer[6], data_buffer[7]]));
            
            if png.width.unwrap() == 0 || png.height.unwrap() == 0 {
                return Err(Error::new(ErrorKind::InvalidData, "Image width and height must be greater than 0"));
            }
            
            let pt = match data_buffer[8] {
                0 => PixelType::BW,
                1 => PixelType::GRAYSCALE,
                2 => PixelType::PALETTE,
                3 => PixelType::TRUECOLOR,
                _ => {
                    return Err(Error::new(ErrorKind::InvalidData, format!("Unknown pixel type: {}", data_buffer[8])));
                }
            };
            
            if png.palette.is_some() && pt != PixelType::PALETTE {
                return Err(Error::new(ErrorKind::InvalidData, format!("A palette block cannot be present in an image with pixel type {:?}", pt)));
            }
            
            png.pixel_type = Some(pt);
        },
        BlockType::P => {
            if png.palette.is_some() {
                return Err(Error::new(ErrorKind::InvalidData, "One file cannot contain multiple palette blocks"));
            }
            
            if length % 3 != 0 {
                return Err(Error::new(ErrorKind::InvalidData, "Palette block length must be a multiple of 3"));
            }
            
            if png.pixel_type.as_ref().is_some_and(|p: &PixelType| -> bool {p != &PixelType::PALETTE}) {
                return Err(Error::new(ErrorKind::InvalidData, format!("A palette block cannot be present in an image with pixel type {:?}", png.pixel_type.as_ref().unwrap())));
            }
            
            let mut palette = Vec::new();
            
            for i in 0..(length / 3) {
                palette.push((data_buffer[(i * 3) as usize], data_buffer[(i * 3 + 1) as usize], data_buffer[(i * 3 + 2) as usize]));
            }
            
            png.palette = Some(palette);
        }
    }
    
    return Ok(());
}

// Parses a Minipng struct into an Image struct
fn parse_minipng(png: &Minipng) -> Result<Image, Error> {
    // parse_block should have checked that the minipng is valid, so we will consider that to be the case and unwrap the options here
    // if the previous code is correct, these unwraps should never panic
    
    let size = (png.width.unwrap(), png.height.unwrap());
    let comments = png.comments.clone();
    let mut pixels = Vec::new();
    
    let n_pixels = size.0 * size.1;
    
    for _ in 0..n_pixels {
        pixels.push(Pixel::new((0, 0, 0)));
    }
    
    match png.pixel_type.as_ref().unwrap() {
        PixelType::BW => {
            let expected_length = (n_pixels as f32 / 8 as f32).ceil() as u32;
            let actual_length = png.data.len() as u32;
        
            if actual_length != expected_length {
                return Err(Error::new(ErrorKind::InvalidData, format!("Data block length must be equal to ceil((width * height) / 8) for pixel type BW (expected {}, got {})", expected_length, actual_length)));
            }
            
            for i in 0..actual_length {
                let b = png.data[i as usize];
                
                for j in 0..8 {
                    if (b & (1 << (7 - j))) != 0 {
                        pixels[i as usize * 8 + j as usize] = Pixel::new((255, 255, 255));
                    }
                }
            }
        },
        PixelType::GRAYSCALE => {
            todo!();
        },
        PixelType::PALETTE => {
            todo!();
        },
        PixelType::TRUECOLOR => {
            todo!();
        }
    }
    
    return Ok(Image::new(
        size,
        pixels,
        comments,
    ));
}