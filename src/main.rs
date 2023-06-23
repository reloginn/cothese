use clap::{App, Arg};
use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
};
use self::compressor::Compressor;

mod kinds {
    pub mod all;
    pub mod png;
    pub mod webp;
}
mod compressor;
mod trash;

type Dir = Mutex<Arc<PathBuf>>;
type IterMutex = Mutex<usize>;

fn main() {
    let matches = App::new("image-compressor")
        .version("0.0.1")
        .author("reloginn")
        .about("Эта программа сжимает все изображения в исходной папке и ложит успешно сжатые изображения в конечную папку")
        .arg(Arg::with_name("input_path").required(true))
        .arg(Arg::with_name("output_path").required(true))
        .arg(Arg::with_name("type").default_value("all").required(false))
        .arg(Arg::with_name("logs").default_value("true").required(false)).get_matches();
    let logs = match matches.value_of("logs").expect("Cannot match the logs") {
        "true" => true,
        "false" => false,
        _ => panic!("Неизвестный параметр логирования, укажите true либо false"),
    };
    let compressor = Compressor::new(matches.value_of("input_path").unwrap(), matches.value_of("output_path").unwrap(), logs).expect("Cannot create a compressor struct");
    match matches.value_of("type").expect("Cannot match the type") {
        "all" => compressor.to().all().compress(),
        "webp" => compressor.to().webp().compress(),
        "png" => compressor.to().png().compress(),
        _ => panic!("Неизвестный тип сжатия"),
    };
}
