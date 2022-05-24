// vigenere cipher functions
use std::io::{stdout, Write};
use std::time::{Duration, Instant};
struct Keyword {
    vec: Vec<u8>,
}

impl Iterator for Keyword {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let mut s = String::from("");

        for n in &self.vec {
            if n >= &26u8 {
                return None;
            }

            let c = (n + 65) as char;
            s.push(c);
        }

        // increment the vector(s)

        for ix in (0..self.vec.len()).rev() {
            self.vec[ix] += 1;
            if ix > 0 && self.vec[ix] >= 26 {
                self.vec[ix] = 0;
            } else {
                break;
            }
        }

        Some(s)
    }
}

pub fn encode(key: &str, input: &str) -> String {
    if !input.is_ascii() {
        return String::from(input);
    }

    let mut out = String::new();
    let key_chars: Vec<char> = String::from(key)
        .to_uppercase()
        .chars()
        .filter(|x| x.is_alphabetic())
        .collect();
    let key_len = key_chars.len();

    if key_len == 0 {
        return String::from(input);
    }

    let mut i: usize = 0;
    for mut c in input.chars() {
        c.make_ascii_uppercase();

        if c.is_alphabetic() {
            let mut c2 = c as u8 + (key_chars[i % key_len] as u8 - 'A' as u8);
            if c2 > 'Z' as u8 {
                c2 -= 26;
            }
            out.push(c2 as char);
            i += 1;
        } else {
            out.push(c);
        }
    }

    out
}

pub fn decode(key: &str, input: &str) -> String {
    if !input.is_ascii() {
        return String::from(input);
    }

    let mut out = String::new();
    let key_chars: Vec<char> = String::from(key).to_uppercase().chars().collect();
    let key_len = key_chars.len();

    let mut i: usize = 0;
    for mut c in input.chars() {
        c.make_ascii_uppercase();

        if c.is_alphabetic() {
            let mut c2 = c as u8 - (key_chars[i % key_len] as u8 - 'A' as u8);
            if c2 < 'A' as u8 {
                c2 += 26;
            }
            out.push(c2 as char);
            i += 1;
        } else {
            out.push(c);
        }
    }

    out
}

pub fn brute_force(len: i32, input: &str) -> String {
    let lower: i32;
    let upper: i32;
    let mut best_chi: f32 = 9999999999999.;
    let mut best = String::from("");

    if len > 0 {
        lower = len;
        upper = len;
    } else if len < 0 {
        lower = 1;
        upper = len * -1;
    } else {
        return best;
    }

    // measure time so we can periodically report where we are in the iterations below
    let one_second = Duration::from_millis(100);
    let mut now = Instant::now();
    let mut buf = String::from("");
    let mut stdout = stdout();

    for i in lower..=upper {
        let key = Keyword {
            vec: vec![0; i as usize],
        };

        for s in key {
            if now.elapsed() > one_second {
                // erase previous output (backspace)
                let erase = (8 as char).to_string().repeat(buf.len());
                print!("{}", erase);

                // write current output
                buf = s.clone();
                print!("{}", buf);
                stdout.flush().unwrap();

                now = Instant::now();
            }

            let decoded = decode(s.as_str(), &input);
            let chi = crate::stats::chi_squared(decoded.as_str(), &crate::english::FREQUENCY);
            if chi < best_chi {
                best_chi = chi;
                best = s;
            }
        }
    }

    if buf.len() > 0 {
        println!("");
    }

    best
}
