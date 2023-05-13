use crate::trash::{compress_jpeg_to_webp, generate_random_name};
use std::{path::PathBuf, sync::Arc, fs};

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
                        compress_jpeg_to_webp(
                            path,
                            self.output_dir
                                .join(format!("{}", generate_random_name()))
                                .with_extension("webp"),
                        )
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
