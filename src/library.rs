use walkdir::WalkDir;
use std::path::{Path, PathBuf};
use image::{DynamicImage, imageops::resize, imageops::FilterType::Triangle};
use webp::{WebPMemory, Encoder};
use std::time::Instant;
#[derive(Debug)]
pub struct Compressor<'a> {
    pub input_directory: Box<&'a Path>,
    pub output_directory: Box<&'a Path>,
    iterator: u16,
}

impl<'a> Compressor<'a> {
    pub fn new(input_directory: &'static str, output_directory: &'static str) -> Self {
        let input_path = Path::new(input_directory);
        if !input_path.exists() {
            panic!("Указаной входной директории не существует, укажите другую, либо создайте эту: {:?}", input_path) 
        }
        let output_path = Path::new(output_directory);
        if !output_path.exists() {
            panic!("Указанной выходной директории не существует, укажите другую, либо создайте эту: {:?}", output_path)
        }
        Self { input_directory: Box::new(input_path), output_directory: Box::new(output_path), iterator: 0 }
    }
    pub async fn compress_of_images(mut self) {
        let file_paths = &self.get_all_of_files_in_directory().await;
        for file in file_paths.iter() {
            let path = Path::new(file);
            if path.is_file() {
                match path.extension().unwrap_or_default().to_str().unwrap() {
                    "png" | "jpg" | "jpeg" => {
                        self.iterator += 1;
                        convert(Box::new(path), Box::new(&self.output_directory.join(format!("{}", self.iterator)).with_extension("webp"))).await;
                    }
                    _ => ()
                }
            }
        }
        if self.iterator <= 0 {
            println!("В заданной директории не было никаких картинок формата JPG, PNG, JPEG, ничего не затронуто");
        } else {
            println!("Успешно конвертировано {} картинок", self.iterator);
        }
    }
    async fn get_all_of_files_in_directory(&self) -> Vec<PathBuf> {
        let mut file_paths: Vec<PathBuf> = Vec::new();

        for entry in WalkDir::new(&self.input_directory.as_ref())
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

async fn convert(input_path: Box<&Path>, output_path: Box<&PathBuf>) {
    let start = Instant::now();
    println!("Начинаю конвертировать файл {:?}...", &input_path);
    let img = image::open(input_path.as_ref()).unwrap();
    let (w, h) = (img.width(), img.height());
    let size_factor = 1.0;
    let img: DynamicImage = image::DynamicImage::ImageRgba8(resize(
        &img,
        (w as f64 * size_factor) as u32,
        (h as f64 * size_factor) as u32,
        Triangle,
    ));
    let encoder: Encoder = Encoder::from_image(&img).unwrap();
    let webp: WebPMemory = encoder.encode(90f32);
    std::fs::write(&output_path.as_ref(), &*webp).unwrap();
    println!("Файл успешно конвертирован за {:?}", start.elapsed());
}