use std::{fs, path::PathBuf, process::exit};

use crate::trash::compress_to_webp;

pub struct Compressor {
    input_dir: PathBuf,
    output_dir: PathBuf,
    logs: bool,
    iter: usize,
}

impl Compressor {
    pub fn new(input_dir: PathBuf, output_dir: PathBuf, logs: bool) -> Self {
        Self {
            input_dir,
            output_dir,
            logs,
            iter: 0,
        }
    }
    pub fn run(mut self) {
        let entries = fs::read_dir(self.input_dir).unwrap_or_else(|err| {
            eprintln!("Возникла ошибка при чтении директории: {}", err);
            exit(1)
        });
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() {
                if let "png" | "jpeg" | "jpg" = path.extension().unwrap().to_str().unwrap() {
                    self.iter += 1;
                    compress_to_webp(
                        path,
                        self.output_dir
                            .join(format!("{}", self.iter))
                            .with_extension("webp"),
                        self.logs,
                    )
                }
            }
        }
    }
}
