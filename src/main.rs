// vsolve
// vigenere solver

mod english;
mod stats;
mod vigenere;
use getopts::Options;
use std::env;

// use getopts::Options;
fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options] [key] input-string", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optflag("d", "decode", "decode input");
    opts.optflagopt("b", "brute", "bruteforce key [optional len]", "LEN");

    let m = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => {
            println!("{}", f.to_string());
            print_usage(&program, opts);
            return;
        }
    };

    if m.opt_present("b") {
        if m.free.len() == 0 {
            print_usage(&program, opts);
            return;
        }

        let input = m.free[0..].join(" ");

        if input.is_empty() {
            print_usage(&program, opts);
            return;
        }

        let brute_len = m.opt_get_default::<i32>("b", -7).unwrap();
        let best = vigenere::brute_force(brute_len, &input);
        let plaintext = vigenere::decode(best.as_str(), &input);
        let chi = stats::chi_squared(&plaintext, &english::FREQUENCY);

        println!("{} [{}] {}", chi, best, plaintext);
        return;
    }

    if m.free.len() < 2 {
        print_usage(&program, opts);
        return;
    }

    let key = &m.free[0];
    let input = m.free[1..].join(" ");

    if input.is_empty() || key.is_empty() {
        print_usage(&program, opts);
        return;
    }

    if !(key.chars().all(char::is_alphabetic)) {
        println!("Key '{}' should not contain non-alphabetic characters", key);
        return;
    }

    let result = if m.opt_present("d") {
        vigenere::decode(key, &input)
    } else {
        vigenere::encode(key, &input)
    };

    println!("{}", result);
    let chi = stats::chi_squared(result.as_str(), &english::FREQUENCY);
    println!("chi: {}", chi);
}
