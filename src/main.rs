use std::process::exit;

use compressor::Compressor;

use crate::args_parser::parse_args;

mod args_parser;
mod compressor;
mod consts;
mod threadpool;
mod trash;

fn main() {
    match parse_args() {
        Ok(appargs) => {
            if let Some(quality) = appargs.quality {
                if quality < 10.0 || quality > 100.0 {
                    eprintln!("Возникла ошибка: Качество не может быть ниже 10.0 и выше 100.0");
                    exit(1)
                }
            }
            Compressor::new(
                appargs.input,
                appargs.output,
                appargs.logs.unwrap_or(true),
                appargs.threads.unwrap_or(4usize),
                appargs.quality.unwrap_or(90f32),
            )
            .run();
        }
        Err(err) => {
            eprintln!("Возникла ошибка: {}", err);
            exit(1)
        }
    };
}
