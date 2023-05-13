use crate::{trash::{generate_random_name, quantize_png}, Dir};
use std::fs;

#[derive(Debug)]
pub struct PNG {
    pub(crate) input_dir: Dir,
    pub(crate) output_dir: Dir,
}

impl PNG {
    pub fn compress(&self) {
        let entries = fs::read_dir(&*self.input_dir).expect("Не могу прочитать директорию");
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_file() {
                    if let "png" = path.extension().unwrap().to_str().unwrap() {
                        quantize_png(
                            path,
                            self.output_dir
                                .join(format!("{}", generate_random_name()))
                                .with_extension("png"),
                        )
                    }
                }
            }
        }
    }
}
