// vigenere cipher functions

pub fn encode(input: &str, key: &str) -> String {
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

pub fn decode(input: &str, key: &str) -> String {
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
