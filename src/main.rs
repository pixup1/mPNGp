extern crate getopts;
use getopts::Options;
use std::{env, path::Path, process::exit};

mod parser;
mod minipng;
mod image;

use parser::parse_file;

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    
    opts.optflag("h", "help", "print this help menu");
    opts.optopt("s", "size", "display the image at a larger size than 1 pixel per pixel", "SIZE");
    
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => {
            println!("Error while parsing arguments: {}", f.to_string());
            return;
        }
    };
    
    if matches.opt_present("h") || matches.free.is_empty() {
        print_usage(&program, opts);
        return;
    }
    
    let pixel_size = match matches.opt_str("s") {
        Some(s) => {
            match s.parse::<u32>() {
                Ok(n) => n,
                Err(e) => {
                    println!("Error while parsing size argument: {}", e);
                    exit(1);
                }
            }
        },
        None => 1,
    };
    
    let filepath = Path::new(&matches.free[0]);
    
    let img = match parse_file(filepath) {
        Ok(i) => { i },
        Err(e) => {
            println!("Error while parsing file: {}", e);
            exit(1);
        }
    };
    
    println!();
    if img.is_bw() {
        img.print_bw();
    } else {
        img.display_color(pixel_size);
    }
}
