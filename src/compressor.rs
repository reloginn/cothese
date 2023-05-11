#![allow(unused)]

use image::{
    imageops::{resize, FilterType::Triangle},
    DynamicImage,
};
use std::{
    fmt::Display,
    path::{Path, PathBuf},
    sync::Arc,
    time::Instant,
};
use webp::{Encoder, WebPMemory};

use crate::kinds::{png::PNG, webp::WebP};

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
    input_dir: Arc<PathBuf>,
    output_dir: Arc<PathBuf>,
}

#[derive(Debug)]
pub struct FromCompressor {
    input_dir: Arc<PathBuf>,
    output_dir: Arc<PathBuf>,
}

impl Compressor {
    pub fn new<T>(input_directory: T, output_directory: T) -> Result<Self, SelfErrors<String>>
    where
        T: Display,
        PathBuf: From<T>,
    {
        let (input_dir, output_dir) = (
            Arc::new(PathBuf::from(input_directory)),
            Arc::new(PathBuf::from(output_directory)),
        );
        if !input_dir.exists() || !output_dir.exists() {
            return Err(SelfErrors::InvalidDirectory(String::from(
                "Одна из директорий невалидна, проверьте правильность директорий",
            )));
        }
        Ok(Self {
            input_dir,
            output_dir,
        })
    }
    pub fn from(self) -> FromCompressor {
        FromCompressor {
            input_dir: self.input_dir,
            output_dir: self.output_dir,
        }
    }
}

impl FromCompressor {
    pub fn webp(self) -> WebP {
        WebP {
            input_dir: self.input_dir,
            output_dir: self.output_dir,
        }
    }
    pub fn png(self) -> PNG {
        PNG {
            input_dir: self.input_dir,
            output_dir: self.output_dir,
        }
    }
}
