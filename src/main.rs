extern crate getopts;
use getopts::Options;
use std::{env, path::Path};

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
    
    let filepath = Path::new(&matches.free[0]);
    
    let img = match parse_file(filepath) {
        Ok(i) => { i },
        Err(e) => {
            println!("Error while parsing file: {}", e);
            return;
        }
    };
    
    println!();
    img.print_bw();
}
