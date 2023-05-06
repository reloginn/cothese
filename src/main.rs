use self::library::Compressor;

mod library;

#[tokio::main]
async fn main() {
    let images = Compressor::new("./input_images/", "./output_images/");
    images.compress_of_images().await;
}
