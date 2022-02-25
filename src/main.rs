// vsolve
// vigenere solver

mod english;
mod stats;
mod vigenere;

fn main() {
    let mut pt = String::from("The quick brown fox");
    let key = "caesar";

    for _ in 0..1000000 {
        pt = vigenere::encode(&pt, key);
    }
}
