use std::{ffi::OsStr, path::PathBuf, process::exit};

use crate::consts::{HELP_EN, HELP_RU, VERSION_EN, VERSION_RU};

pub struct AppArgs {
    pub input: PathBuf,
    pub output: PathBuf,
    pub logs: Option<bool>,
    pub threads: Option<usize>,
    pub quality: Option<f32>,
}

pub fn parse_args() -> Result<AppArgs, pico_args::Error> {
    let mut pargs = pico_args::Arguments::from_env();
    match pargs.contains(["-h", "--help"]) {
        true => match pargs.contains("--lang=ru") {
            true => {
                print!("{}", HELP_RU);
                exit(0)
            }
            false => {
                print!("{}", HELP_EN);
                exit(0)
            }
        },
        false => (),
    }
    match pargs.contains(["-v", "--version"]) {
        true => match pargs.contains("--lang=ru") {
            true => {
                print!("{}", VERSION_RU);
                exit(0)
            }
            false => {
                print!("{}", VERSION_EN);
                exit(0)
            }
        },
        false => (),
    }

    let args = AppArgs {
        input: pargs
            .value_from_os_str("--input", |s: &OsStr| -> Result<PathBuf, &'static str> {
                Ok(s.into())
            })?,
        output: pargs
            .value_from_os_str("--output", |s: &OsStr| -> Result<PathBuf, &'static str> {
                Ok(s.into())
            })?,
        logs: pargs.opt_value_from_str("--logs")?,
        threads: pargs.opt_value_from_str("--threads")?,
        quality: pargs.opt_value_from_str("--quality")?,
    };
    Ok(args)
}
