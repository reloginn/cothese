use std::{
    fmt::Display,
    path::PathBuf,
    sync::{Arc, Mutex},
};

use crate::{
    kinds::{all::All, png::PNG, webp::WebP},
    Dir, IterMutex,
};

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
    _logs: bool,
    iter: IterMutex,
}

#[derive(Debug)]
pub struct To {
    input_dir: Dir,
    output_dir: Dir,
    _logs: bool,
    iter: IterMutex,
}

impl Compressor {
    pub fn new<T, U>(
        input_directory: T,
        output_directory: U,
        logs: bool,
    ) -> Result<Self, SelfErrors<String>>
    where
        T: Display,
        U: Display,
        PathBuf: From<T>,
        PathBuf: From<U>,
    {
        let (input_dir, output_dir) = (
            Mutex::new(Arc::new(PathBuf::from(input_directory))),
            Mutex::new(Arc::new(PathBuf::from(output_directory))),
        );
        if !input_dir.lock().unwrap().exists() || !output_dir.lock().unwrap().exists() {
            return Err(SelfErrors::Error(String::from(
                "Одна из директорий невалидна, проверьте правильность директорий",
            )));
        }
        Ok(Self {
            input_dir,
            output_dir,
            _logs: logs,
            iter: Mutex::new(0),
        })
    }
    pub fn to(self) -> To {
        To {
            input_dir: self.input_dir,
            output_dir: self.output_dir,
            _logs: self._logs,
            iter: self.iter,
        }
    }
}

impl To {
    pub fn webp(self) -> WebP {
        WebP {
            input_dir: self.input_dir,
            output_dir: self.output_dir,
            _logs: self._logs,
            iter: self.iter,
        }
    }
    pub fn png(self) -> PNG {
        PNG {
            input_dir: self.input_dir,
            output_dir: self.output_dir,
            _logs: self._logs,
            iter: self.iter,
        }
    }
    pub fn all(self) -> All {
        All {
            input_dir: self.input_dir,
            output_dir: self.output_dir,
            _logs: self._logs,
            iter: self.iter,
        }
    }
}
