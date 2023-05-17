use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
};

pub use self::compressor::Compressor;

mod kinds {
    pub mod all;
    pub mod png;
    pub mod webp;
}
mod compressor;
mod trash;

type Dir = Mutex<Arc<PathBuf>>;
type IterMutex = Mutex<usize>;
