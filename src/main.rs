use self::library::Images;

mod library;

#[tokio::main]
async fn main() {
    let images = Images::new("./input_images/", "./output_images/");
    images.compress_of_images().await;
}
