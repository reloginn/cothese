use std::{fmt::Display, path::PathBuf, sync::Arc};

use crate::{kinds::{all::All, png::PNG, webp::WebP}, Dir};

#[allow(unused)]
#[derive(Debug)]
pub enum SelfErrors<T>
where
    T: Display,
{
    Error(T),
}

impl<T> Display for SelfErrors<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Error(err) => writeln!(f, "{}", err),
        }
    }
}

#[derive(Debug)]
pub struct Compressor {
    input_dir: Dir,
    output_dir: Dir,
}

#[derive(Debug)]
pub struct FromCompressor {
    input_dir: Dir,
    output_dir: Dir,
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
            return Err(SelfErrors::Error(String::from(
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
    pub fn all(self) -> All {
        All {
            input_dir: self.input_dir,
            output_dir: self.output_dir,
        }
    }
}
