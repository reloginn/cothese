#![allow(unused)]

use std::fs;
use crate::generate_random_name;
use std::{
    borrow::Borrow,
    path::{Path, PathBuf},
    sync::Arc,
    time::Instant,
};

use image::{
    imageops::{resize, FilterType::Triangle},
    DynamicImage,
};
use webp::{Encoder, WebPMemory};

#[derive(Debug)]
pub struct WebP {
    pub(crate) input_dir: Arc<PathBuf>,
    pub(crate) output_dir: Arc<PathBuf>,
}

impl WebP {
    pub fn compress(&self) {
        let entries = fs::read_dir(&*self.input_dir).expect("Не могу прочитать директорию");
        let mut count: usize = 0;
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_file() {
                    if let "jpg" | "jpeg" | "png" = path.extension().unwrap().to_str().unwrap() {
                        count += 1;
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
                        std::fs::write(
                            &self
                                .output_dir
                                .join(format!("{}", generate_random_name()))
                                .with_extension("webp"),
                            &*webp,
                        )
                        .unwrap();
                        println!("Файл успешно конвертирован за {:?}", start.elapsed());
                    }
                }
            }
        }
        if count <= 0 {
            println!("В заданной директории не было JPEG, PNG картинок, ничего не затронуто");
        } else {
            println!("Успешно конвертировано {} картинок", count)
        }
    }
}
