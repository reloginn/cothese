use image::{imageops::resize, imageops::FilterType::Triangle, DynamicImage};
use std::{
    fmt::Display,
    path::{Path, PathBuf},
    sync::Arc,
    time::Instant,
};
use walkdir::WalkDir;
use webp::{Encoder, WebPMemory};

#[allow(unused)]
#[derive(Debug)]
pub enum SelfErrors<T>
where
    T: Display,
{
    InvalidDirectory(T),
    Other(T),
}

impl<T> Display for SelfErrors<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidDirectory(err) => writeln!(f, "{}", err),
            Self::Other(err) => writeln!(f, "{}", err),
        }
    }
}

#[derive(Debug)]
pub struct Compressor {
    input_directory: Arc<PathBuf>,
    output_directory: Arc<PathBuf>,
}

impl Compressor {
    pub fn new(
        input_directory: &'static str,
        output_directory: &'static str,
    ) -> Result<Self, SelfErrors<String>> {
        let (input_path, output_path) = (
            PathBuf::from(input_directory),
            PathBuf::from(output_directory),
        );
        if !input_path.exists() || !output_path.exists() {
            return Err(SelfErrors::InvalidDirectory(String::from(
                "Одна из директорий невалидна",
            )));
        }
        Ok(Self {
            input_directory: Arc::new(input_path),
            output_directory: Arc::new(output_path),
        })
    }
    pub fn compress_of_images(&self) {
        let file_paths = &self.get_all_of_files_in_directory();
        let mut iter: u16 = 0;
        for file in file_paths.iter() {
            let path = Path::new(file);
            if path.is_file() {
                match path.extension().unwrap_or_default().to_str().unwrap() {
                    "png" | "jpg" | "jpeg" => {
                        iter += 1;
                        convert(
                            path,
                            &self
                                .output_directory
                                .join(format!("{}", iter))
                                .with_extension("webp"),
                        );
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

        for entry in WalkDir::new(&*self.input_directory)
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

fn convert(path_to_file: &Path, output_path: &Path) {
    let start = Instant::now();
    println!("Начинаю конвертировать файл {:?}...", &path_to_file);
    let image = image::open(path_to_file).unwrap();
    let (width, height) = (image.width(), image.height());
    let size_factor = 1.0;
    let img: DynamicImage = image::DynamicImage::ImageRgba8(resize(
        &image,
        (width as f64 * size_factor) as u32,
        (height as f64 * size_factor) as u32,
        Triangle,
    ));
    let encoder: Encoder = Encoder::from_image(&img).unwrap();
    let webp: WebPMemory = encoder.encode(90f32);
    std::fs::write(output_path, &*webp).unwrap();
    println!("Файл успешно конвертирован за {:?}", start.elapsed());
}
