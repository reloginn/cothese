use std::{ffi::OsStr, path::PathBuf, process::exit};

use crate::consts::{HELP, VERSION};

pub struct AppArgs {
    pub input: PathBuf,
    pub output: PathBuf,
    pub logs: Option<bool>,
    pub threads: Option<usize>,
    pub quality: Option<f32>,
}

pub fn parse_args() -> Result<AppArgs, pico_args::Error> {
    let mut pargs = pico_args::Arguments::from_env();
    if pargs.contains(["-h", "--help"]) {
        print!("{}", HELP);
        exit(0);
    }
    if pargs.contains(["-v", "--version"]) {
        print!("{}", VERSION);
        exit(0);
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
