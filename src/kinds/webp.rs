#![allow(unused)]

use std::{
    path::{Path, PathBuf},
    sync::Arc,
    time::Instant,
};

use image::{
    imageops::{resize, FilterType::Triangle},
    DynamicImage,
};
use walkdir::WalkDir;
use webp::{Encoder, WebPMemory};

#[derive(Debug)]
pub struct WebP {
    pub(crate) input_dir: Arc<PathBuf>,
    pub(crate) output_dir: Arc<PathBuf>,
}

impl WebP {
    pub fn compress(&self) {
        let file_paths = &self.get_all_of_files_in_directory();
        let mut iter: u16 = 0;
        for file in file_paths.iter() {
            let path = Path::new(file);
            if path.is_file() {
                match path.extension().unwrap_or_default().to_str().unwrap() {
                    "png" | "jpg" | "jpeg" => {
                        iter += 1;
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
                                .join(format!("{}", iter))
                                .with_extension("webp"),
                            &*webp,
                        )
                        .unwrap();
                        println!("Файл успешно конвертирован за {:?}", start.elapsed());
                    }
                    _ => (),
                }
            }
        }
        if iter <= 0 {
            println!("В заданной директории не было JPEG, PNG картинок, ничего не затронуто");
        } else {
            println!("Успешно конвертировано {} картинок", iter)
        }
    }
    fn get_all_of_files_in_directory(&self) -> Vec<PathBuf> {
        let mut file_paths: Vec<PathBuf> = Vec::new();

        for entry in WalkDir::new(&*self.input_dir)
            .max_depth(1)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            if entry.file_type().is_file() {
                file_paths.push(entry.path().to_path_buf());
            }
        }
        file_paths
    }
}
