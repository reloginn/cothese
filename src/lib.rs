use std::{
    path::PathBuf,
    process::{exit, Command},
    time::Instant,
};

pub use self::compressor::Compressor;
use image::{
    imageops::{resize, FilterType::Triangle},
    DynamicImage,
};
use rand::Rng;
use webp::{Encoder, WebPMemory};

const ASCII: &str = "qwertyuiopasdfghjklzxcvbnmQWERTYUIOPASDFGHJKLZXCVBNM";
const NUMBERS: &str = "0123456789";
const MAX_ITERATIONS: u8 = 8;

mod kinds {
    pub mod all;
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

fn quantize_png(path: PathBuf, output: PathBuf) {
    let install = Command::new("cargo")
        .args(["install", "pngquant"])
        .status()
        .unwrap_or_else(|err| {
            eprintln!("Возникла ошибка: {}", err);
            exit(0)
        });
    if !install.success() {
        panic!("Операция скачивания pngquant завершилась неудачно");
    }
    let quantize = Command::new("pngquant")
        .args([
            "--force",
            path.to_str().unwrap(),
            "--output",
            output.to_str().unwrap(),
        ])
        .status()
        .unwrap_or_else(|err| {
            eprintln!("Возникла ошибка: {}", err);
            exit(0)
        });
    if !quantize.success() {
        panic!("Операция квантинизации завершилась неудачно")
    }
}

fn compress_jpeg_to_webp(path: PathBuf, output: PathBuf) {
    let start = Instant::now();
    println!("Начинаю конвертировать файл {:?}...", &path);
    let image = image::open(path).unwrap();
    let (width, height) = (image.width(), image.height());
    const SIZE_FACTOR: f64 = 1.0;
    let img: DynamicImage = image::DynamicImage::ImageRgba8(resize(
        &image,
        (width as f64 * SIZE_FACTOR) as u32,
        (height as f64 * SIZE_FACTOR) as u32,
        Triangle,
    ));
    let encoder: Encoder = Encoder::from_image(&img).unwrap();
    let webp: WebPMemory = encoder.encode(90f32);
    std::fs::write(output, &*webp).unwrap();
    println!("Файл успешно конвертирован за {:?}", start.elapsed());
}
