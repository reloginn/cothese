use self::library::Compressor;

mod library;

fn main() {
    let images = Compressor::new("./input_images/", "./output_images/").unwrap();
    images.compress_of_images();
}
