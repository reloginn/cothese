#![allow(clippy::ptr_arg)]

use std::process::exit;

use compressor::Compressor;

use crate::app::parse_args;

mod app;
mod compressor;
mod consts;
mod trash;

fn main() {
    match parse_args() {
        Ok(appargs) => {
            let logs = appargs.logs.unwrap_or(true);
            Compressor::new(appargs.input, appargs.output, logs).run();
        }
        Err(err) => {
            eprintln!("Возникла ошибка: {}", err);
            exit(1)
        }
    };
}
