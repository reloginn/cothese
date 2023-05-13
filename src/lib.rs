use std::{sync::Arc, path::PathBuf};

pub use self::compressor::Compressor;

mod kinds {
    pub mod all;
    pub mod png;
    pub mod webp;
}
mod compressor;
mod trash;

type Dir = Arc<PathBuf>;
