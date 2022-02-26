// vsolve
// vigenere solver

mod english;
mod stats;
mod vigenere;
use getopts::Options;
use std::env;

// use getopts::Options;
const USAGE: &str = "vsolve key input-string";

fn main() {
    let args: Vec<String> = env::args().collect();
    // let _program = &args[0];

    let mut opts = Options::new();
    opts.optflag("d", "decode", "decode input");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => {
            panic!("{}", f.to_string())
        }
    };

    if matches.free.len() < 2 {
        println!("Usage: {}", USAGE);
        std::process::exit(1);
    }

    let key = &matches.free[0];
    let input = matches.free[1..].join(" ");

    if input.is_empty() || key.is_empty() {
        println!("Usage: {}", USAGE);
        std::process::exit(1);
    }

    let result = if matches.opt_present("d") {
        vigenere::decode(key, &input)
    } else {
        vigenere::encode(key, &input)
    };

    println!("{}", result);
}
