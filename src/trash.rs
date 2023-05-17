use std::{
    path::PathBuf,
    process::{exit, Command},
    time::Instant,
};

use image::{
    imageops::{resize, FilterType::Triangle},
    DynamicImage,
};
use webp::{Encoder, WebPMemory};

pub(crate) fn quantize_png(path: PathBuf, output: PathBuf) {
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

pub(crate) fn compress_jpeg_to_webp(path: PathBuf, output: PathBuf) {
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
