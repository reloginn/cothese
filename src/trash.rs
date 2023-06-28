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

fn quantize_it(path: PathBuf, output: PathBuf) {
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

pub fn quantize_a_png_image(path: PathBuf, output: PathBuf, enable_logs: bool) {
    if enable_logs {
        let start = Instant::now();
        println!(".png => .png...");
        quantize_it(path, output);
        println!(".png => .png: {:?}", start.elapsed());
    } else {
        quantize_it(path, output)
    }
}

fn compress_it(path: PathBuf, output: PathBuf) {
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
}

pub fn compress_jpeg_to_webp(path: PathBuf, output: PathBuf, enable_logs: bool) {
    if enable_logs {
        let start = Instant::now();
        println!(".jpg, .jpeg, ... => .webp...");
        compress_it(path, output);
        println!(".jpg, .jpeg, ... => .webp: {:?}", start.elapsed());
    } else {
        compress_it(path, output)
    }
}

pub fn collect_args() -> Vec<String> {
    let mut args = Vec::new();
    for (index, value) in std::env::args().enumerate() {
        if index != 0 {
            args.push(value);
        }
    }
    args
}

pub fn value_in_vec(value: &str, slice: &Vec<String>) -> bool {
    for v in slice.iter() {
        if *v == value.to_string() {
            return true;
        }
    }
    false
}

pub fn value_in_vec_with_index(value: &str, slice: &Vec<String>) -> Option<usize> {
    for (i, v) in slice.into_iter().enumerate() {
        if *v == value.to_string() {
            return Some(i);
        }
    }
    None
}