use std::{path::PathBuf, process::exit, time::Instant};

use image::{
    imageops::{resize, FilterType::Triangle},
    DynamicImage,
};
use webp::{Encoder, WebPMemory};

fn compress_it(path: PathBuf, output: PathBuf) {
    let image = image::open(path).unwrap_or_else(|err| {
        eprintln!("Возникла ошибка при открытии изображения: {}", err);
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
        eprintln!("Возникла ошибка при создании энкодера: {}", err);
        exit(1)
    });
    let webp: WebPMemory = encoder.encode(90f32);
    std::fs::write(output, &*webp).unwrap_or_else(|err| {
        eprintln!("Возникла ошибка при записи готового изображения: {}", err);
        exit(1)
    });
}

pub fn compress_to_webp(path: PathBuf, output: PathBuf, logs: bool) {
    if logs {
        let start = Instant::now();
        println!(".png, .jpg, .jpeg => .webp...");
        compress_it(path, output);
        println!(".png, .jpg, .jpeg => .webp: {:?}", start.elapsed());
    } else {
        compress_it(path, output)
    }
}