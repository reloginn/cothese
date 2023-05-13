pub use self::compressor::Compressor;

mod kinds {
    pub mod all;
    pub mod png;
    pub mod webp;
}
mod compressor;
mod trash;
