use std::process::exit;

use compressor::Compressor;

use crate::args_parser::parse_args;

mod args_parser;
mod compressor;
mod consts;
mod trash;
mod threadpool;

fn main() {
    match parse_args() {
        Ok(appargs) => {
            Compressor::new(appargs.input, appargs.output, appargs.logs.unwrap_or(true), appargs.threads.unwrap_or(4usize)).run();
        }
        Err(err) => {
            eprintln!("Возникла ошибка: {}", err);
            exit(1)
        }
    };
}
