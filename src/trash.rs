use std::{path::PathBuf, process::exit, time::Instant};

use image::{
    imageops::{resize, FilterType::Triangle},
    DynamicImage,
};
use webp::{Encoder, WebPMemory};

use crate::consts::{DEFAULT, BOLD, RED, BLUE};

fn compress_it(path: PathBuf, output: PathBuf, quality: f32) {
    let image = image::open(path).unwrap_or_else(|err| {
        eprintln!("Error has occured: {}{}{}{}", BOLD, RED, err, DEFAULT);
        exit(1)
    });
    let (width, height) = (image.width(), image.height());
    const SIZE_FACTOR: f64 = 1.0;
    let img: DynamicImage = image::DynamicImage::ImageRgba8(resize(
        &image,
        (width as f64 * SIZE_FACTOR) as u32,
        (height as f64 * SIZE_FACTOR) as u32,
        Triangle,
    ));
    let encoder: Encoder = Encoder::from_image(&img).unwrap_or_else(|err| {
        eprintln!("Error has occured: {}{}{}{}", BOLD, RED, err, DEFAULT);
        exit(1)
    });
    let webp: WebPMemory = encoder.encode(quality);
    std::fs::write(output, &*webp).unwrap_or_else(|err| {
        eprintln!("Error has occured: {}{}{}{}", BOLD, RED, err, DEFAULT);
        exit(1)
    });
}

pub fn compress_to_webp(path: PathBuf, output: PathBuf, logs: bool, quality: f32) {
    if logs {
        let start = Instant::now();
        println!("{}{}[INFORMATION]{} .png, .jpg, .jpeg => .webp...", BOLD, BLUE, DEFAULT);
        compress_it(path, output, quality);
        println!("{}{}[INFORMATION]{} .png, .jpg, .jpeg => .webp in {:?}", BOLD, BLUE, DEFAULT, start.elapsed());
    } else {
        compress_it(path, output, quality)
    }
}
