#![allow(unused)]

use crate::generate_random_name;
use std::fs;
use std::process::exit;
use std::{
    fs::File,
    path::{Path, PathBuf},
    process::Command,
    sync::Arc,
};

#[derive(Debug)]
pub struct PNG {
    pub(crate) input_dir: Arc<PathBuf>,
    pub(crate) output_dir: Arc<PathBuf>,
}

impl PNG {
    pub fn compress(&self) {
        let entries = fs::read_dir(&*self.input_dir).expect("Не могу прочитать директорию");
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();

                if path.is_file() && path.extension().unwrap() == "png" {
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
                            &self
                                .output_dir
                                .join(format!("{}", generate_random_name()))
                                .with_extension("png")
                                .to_str()
                                .unwrap(),
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
            }
        }
    }
}
