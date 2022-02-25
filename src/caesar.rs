// caesar cipher functions

pub fn rotate(c: char, n: u8) -> c {
    if c.is_alphabetic() {
        n += c as u8;
    } else {
        return c;
    }
}
