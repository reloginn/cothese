use std::{fs, path::PathBuf, process::exit};

use crate::{threadpool::ThreadPool, trash::compress_to_webp, consts::{BOLD, RED, DEFAULT}};

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
        let entries = fs::read_dir(self.input_dir).unwrap_or_else(|err| {
            eprintln!("Error has occured: {}{}{}{}", BOLD, RED, err, DEFAULT);
            exit(1)
        });
        for entry in entries.flatten() {
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
        }
    }
}
