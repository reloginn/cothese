#![allow(clippy::ptr_arg)]

use trash::{collect_args, value_in_vec, value_in_vec_with_index};

use crate::consts::{HELP, VERSION};

use self::compressor::Compressor;
use std::{
    path::PathBuf,
    process::exit,
    sync::{Arc, Mutex},
};

mod kinds {
    pub mod all;
    pub mod webp;
}
mod compressor;
mod trash;
mod consts;

type Dir = Mutex<Arc<PathBuf>>;
type IterMutex = Mutex<usize>;

fn main() {
    let args = collect_args();
    if value_in_vec("-h", &args) || value_in_vec("--help", &args) {
        println!("{}", HELP);
        exit(0)
    } else if value_in_vec("-v", &args) || value_in_vec("--version", &args) {
        println!("{}", VERSION);
        exit(0)
    }
    if let Some(index) = value_in_vec_with_index("--input", &args) {
        if args.len() <= index + 1 {
            eprintln!("Ошибка: После флага --input должен быть путь до директории");
            exit(1);
        } else {
            let input_path = PathBuf::from(&args[index + 1]);
            if input_path.exists() {
                if input_path.is_dir() {
                    if let Some(index) = value_in_vec_with_index("--output", &args) {
                        if args.len() <= index + 1 {
                            eprintln!("Ошибка: После флага --output должен быть путь до директории");
                            exit(1)
                        } else {
                            let output_path = PathBuf::from(&args[index + 1]);
                            if output_path.exists() {
                                if output_path.is_dir() {
                                    if let Some(index) = value_in_vec_with_index("--logs", &args) {
                                        if args.len() <= index + 1 {
                                            eprintln!("Ошибка: Если вы указали флаг --logs, вы должны указать значение");
                                            exit(1)
                                        } else {
                                            match args[index + 1].as_str() {
                                                "true" => {
                                                    const LOGS: bool = true;
                                                    let compressor = Compressor::new(input_path, output_path, LOGS).to();
                                                    if let Some(index) = value_in_vec_with_index("--type", &args) {
                                                        if args.len() <= index + 1 {
                                                            eprintln!("Ошибка: Если вы указали флаг --type, вы должны указать значение");
                                                            exit(1)
                                                        } else {
                                                            match args[index + 1].as_str() {
                                                                "all" => compressor.all(),
                                                                "jpg-webp" => compressor.jpg_to_webp(),
                                                                _ => {
                                                                    eprintln!("Ошибка: --type может иметь только значения all или jpg-webp");
                                                                    exit(1)
                                                                }
                                                            }
                                                        }
                                                    } else {
                                                        compressor.all()
                                                    }
                                                }
                                                "false" => {
                                                    const LOGS: bool = false;
                                                    let compressor = Compressor::new(input_path, output_path, LOGS).to();
                                                    if let Some(index) = value_in_vec_with_index("--type", &args) {
                                                        if args.len() <= index + 1 {
                                                            eprintln!("Ошибка: Если вы указали флаг --type, вы должны указать значение");
                                                            exit(1)
                                                        } else {
                                                            match args[index + 1].as_str() {
                                                                "all" => compressor.all(),
                                                                "jpg-webp" => compressor.jpg_to_webp(),
                                                                _ => {
                                                                    eprintln!("Ошибка: --type может иметь только значения all или jpg-webp");
                                                                    exit(1)
                                                                }
                                                            }
                                                        }
                                                    } else {
                                                        compressor.all()
                                                    }
                                                }
                                                _ => {
                                                    eprintln!("Ошибка: Флаг --logs может иметь только два значения: true или false");
                                                    exit(1)
                                                }
                                            }
                                        }
                                    } else {
                                        const LOGS: bool = true;
                                        let compressor = Compressor::new(input_path, output_path, LOGS).to();
                                        if let Some(index) = value_in_vec_with_index("--type", &args) {
                                            if args.len() <= index + 1 {
                                                eprintln!("Ошибка: Если вы указали флаг --type, вы должны указать значение");
                                                exit(1)
                                            } else {
                                                match args[index + 1].as_str() {
                                                    "all" => compressor.all(),
                                                    "jpg-webp" => compressor.jpg_to_webp(),
                                                    _ => {
                                                        eprintln!("Ошибка: --type может иметь только значения all или jpg-webp");
                                                        exit(1)
                                                    }
                                                }
                                            }
                                        } else {
                                            compressor.all()
                                        }
                                    }
                                } else {
                                    eprintln!("Ошибка: --output: Путь не является папкой");
                                    exit(1)
                                }
                            } else {
                                eprintln!("Ошибка: --output: Такого пути не существует");
                                exit(1)
                            }
                        }
                    } else {
                        eprintln!("Ошибка: Флаг --output является обязательным");
                        exit(1)
                    }
                } else {
                    eprintln!("Ошибка: --input: Путь не является папкой");
                    exit(1)
                }
            } else {
                eprintln!("Ошибка: --input: Такого пути не существует");
                exit(1)
            }
        }
    } else {
        eprintln!("Ошибка: Флаг --input является обязательным");
        exit(1);
    }
}
