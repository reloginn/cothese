#![allow(unused)]

pub use self::compressor::Compressor;
use rand::Rng;

const ASCII: &str = "qwertyuiopasdfghjklzxcvbnmQWERTYUIOPASDFGHJKLZXCVBNM";
const NUMBERS: &str = "0123456789";
const MAX_ITERATIONS: u8 = 8;

mod kinds {
    pub mod png;
    pub mod webp;
}
mod compressor;

fn generate_random_name() -> String {
    let mut result = String::new();
    let ascii: Vec<char> = ASCII.chars().collect();
    let numbers: Vec<char> = NUMBERS.chars().collect();
    for _ in 0..MAX_ITERATIONS {
        let (random_ascii, random_numbers) = (
            rand::thread_rng().gen_range(0..ascii.len()),
            rand::thread_rng().gen_range(0..numbers.len()),
        );
        result.push(ascii[random_ascii]);
        result.push(numbers[random_numbers]);
    }
    result
}
