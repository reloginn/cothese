use crate::{trash::{compress_jpeg_to_webp, generate_random_name, quantize_png}, Dir};
use std::fs;

#[derive(Debug)]
pub struct All {
    pub(crate) input_dir: Dir,
    pub(crate) output_dir: Dir,
}

impl All {
    pub fn compress(&self) {
        let entries = fs::read_dir(&*self.input_dir).expect("Не могу прочитать директорию");
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_file() {
                    match path.extension().unwrap().to_str().unwrap() {
                        "jpeg" | "jpg" => compress_jpeg_to_webp(
                            path,
                            self.output_dir
                                .join(format!("{}", generate_random_name()))
                                .with_extension("webp"),
                        ),
                        "png" => quantize_png(
                            path,
                            self.output_dir
                                .join(format!("{}", generate_random_name()))
                                .with_extension("png"),
                        ),
                        _ => (),
                    }
                }
            }
        }
    }
}
