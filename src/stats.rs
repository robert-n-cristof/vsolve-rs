// statistical functions
use regex::Regex;

// get frequency count of English letters in a str
// as array 0..25 ('A' to 'Z'), case insensitive
pub fn get_english_frequency(input: &str) -> Vec<u32> {
    let mut f = vec![0 as u32; 26];

    for mut c in input.chars() {
        if c.is_alphabetic() {
            c.make_ascii_uppercase();
            let ix = c as usize - 65;
            f[ix] += 1;
        }
    }

    f
}

// Index of Coincidence (normalized; English)
// https://en.wikipedia.org/wiki/Index_of_coincidence
// measure of how likely to draw 2 matching letters in random locations in string
// assuming English alphabet of 26 characters
pub fn _ic(input: &str) -> f32 {
    if input.len() == 0 {
        return 0.;
    }

    let freq = get_english_frequency(&input);

    let mut sum: u32 = 0;
    let mut total: u32 = 0;
    for &i in &freq {
        sum += i * (i - 1);
        total += i;
    }
    let denominator = total * (total - 1);

    freq.len() as f32 * sum as f32 / denominator as f32
}

pub fn chi_squared(input: &str, freq: &[f32]) -> f32 {
    let re = Regex::new(r"[^A-Za-z]").unwrap();
    let reduced_input = re.replace_all(&input, "");

    let found_frequency = get_english_frequency(&reduced_input);
    let len = reduced_input.len() as f32;
    let mut chi: f32 = 0.;

    for (ix, f) in freq.iter().enumerate() {
        let expected = len * f;
        let found = found_frequency[ix] as f32;

        chi += (found * found / expected) - (2.0 * found) + expected;
    }

    chi
}
