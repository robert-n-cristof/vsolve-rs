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

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => {
            println!("{}", f.to_string());
            print_usage(&program, opts);
            return;
        }
    };

    if matches.free.len() < 2 {
        print_usage(&program, opts);
        return;
    }

    let key = &matches.free[0];
    let input = matches.free[1..].join(" ");

    if input.is_empty() || key.is_empty() {
        print_usage(&program, opts);
        return;
    }

    if matches.opt_present("b") {
        let brute_len = matches.opt_get_default::<i32>("b", -7).unwrap();
        let best = vigenere::brute_force(brute_len, &input);
        let plaintext = vigenere::decode(best.as_str(), &input);
        let chi = stats::chi_squared(&plaintext, &english::FREQUENCY);

        println!("{} [{}] {}", chi, best, plaintext);
        return;
    }

    let result = if matches.opt_present("d") {
        vigenere::decode(key, &input)
    } else {
        vigenere::encode(key, &input)
    };

    println!("{}", result);
    let chi = stats::chi_squared(result.as_str(), &english::FREQUENCY);
    println!("chi: {}", chi);
}
