use std::{fs, path::PathBuf, process::exit, time::Instant};

use image::{
    imageops::{resize, FilterType::Triangle},
    DynamicImage,
};
use webp::{Encoder, WebPMemory};

use crate::{
    consts::{BLUE, BOLD, DEFAULT, RED},
    threadpool::ThreadPool,
};

pub struct Compressor {
    input_dir: PathBuf,
    output_dir: PathBuf,
    logs: bool,
    iter: usize,
    threads: usize,
    quality: f32,
}

impl Compressor {
    pub fn new(
        input_dir: PathBuf,
        output_dir: PathBuf,
        logs: bool,
        threads: usize,
        quality: f32,
    ) -> Self {
        Self {
            input_dir,
            output_dir,
            logs,
            iter: 0,
            threads,
            quality,
        }
    }
    pub fn run(mut self) {
        let pool = ThreadPool::new(self.threads);
        fs::read_dir(self.input_dir)
            .unwrap_or_else(|err| {
                eprintln!("Error has occured: {}{}{}{}", BOLD, RED, err, DEFAULT);
                exit(1)
            })
            .flatten()
            .for_each(|entry| {
                let path = entry.path();
                if path.is_file() {
                    if let "png" | "jpeg" | "jpg" = path.extension().unwrap().to_str().unwrap() {
                        let output_dir = self
                            .output_dir
                            .join(format!("{}", self.iter))
                            .with_extension("webp");
                        let logs = self.logs;
                        let quality = self.quality;
                        self.iter += 1;
                        pool.execute(move || compress_to_webp(path, output_dir, logs, quality))
                    }
                }
            });
    }
}

fn compress_to_webp(path: PathBuf, output: PathBuf, logs: bool, quality: f32) {
    if logs {
        let start = Instant::now();
        println!(
            "{}{}[INFORMATION]{} .png, .jpg, .jpeg => .webp...",
            BOLD, BLUE, DEFAULT
        );
        compress_it(path, output, quality);
        println!(
            "{}{}[INFORMATION]{} .png, .jpg, .jpeg => .webp in {:?}",
            BOLD,
            BLUE,
            DEFAULT,
            start.elapsed()
        );
    } else {
        compress_it(path, output, quality)
    }
}

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
