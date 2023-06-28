use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
};

use crate::{
    kinds::{all::All, webp::WebP},
    Dir, IterMutex,
};

#[allow(unused)]
#[derive(Debug)]
pub enum SelfErrors {
    InvalidDirectory,
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
    ) -> Self
    where
        PathBuf: From<T>,
        PathBuf: From<U>,
    {
        let (input_dir, output_dir) = (
            Mutex::new(Arc::new(PathBuf::from(input_directory))),
            Mutex::new(Arc::new(PathBuf::from(output_directory))),
        );
        Self {
            input_dir,
            output_dir,
            _logs: logs,
            iter: Mutex::new(0),
        }
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
    pub fn jpg_to_webp(self) {
        let webp = WebP {
            input_dir: self.input_dir,
            output_dir: self.output_dir,
            _logs: self._logs,
            iter: self.iter,
        };
        webp.compress()
    }
    pub fn all(self) {
        let all = All {
            input_dir: self.input_dir,
            output_dir: self.output_dir,
            _logs: self._logs,
            iter: self.iter,
        };
        all.compress()
    }
}
