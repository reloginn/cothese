use std::process::exit;

use compressor::Compressor;

use crate::{
    args_parser::parse_args,
    consts::{BOLD, DEFAULT, RED, YELLOW},
};

mod args_parser;
mod compressor;
mod consts;
mod threadpool;

fn main() {
    match parse_args() {
        Ok(appargs) => {
            if let Some(quality) = appargs.quality {
                if !(10.0..=100.0).contains(&quality) {
                    eprintln!(
                        "{}{}Quality cannot be lower than 10.0 and higher than 100.0{}",
                        BOLD, RED, DEFAULT
                    );
                    exit(1)
                }
            }
            if let Some(threads) = &appargs.threads {
                if *threads >= 64 {
                    println!("{}{}[WARNING]{} You specified a number of threads greater than 64, this could lead to a problem", BOLD, YELLOW, DEFAULT)
                }
            }
            Compressor::new(
                appargs.input,
                appargs.output,
                appargs.logs.unwrap_or(true),
                appargs.threads.unwrap_or(8),
                appargs.quality.unwrap_or(90f32),
            )
            .run();
        }
        Err(err) => {
            eprintln!("Error has occured: {}{}{}{}", BOLD, RED, err, DEFAULT);
            exit(1)
        }
    };
}
