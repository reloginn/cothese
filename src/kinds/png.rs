use crate::{generate_random_name, quantize_png};
use std::{path::PathBuf, sync::Arc, fs};

#[derive(Debug)]
pub struct PNG {
    pub(crate) input_dir: Arc<PathBuf>,
    pub(crate) output_dir: Arc<PathBuf>,
}

impl PNG {
    pub fn compress(&self) {
        let entries = fs::read_dir(&*self.input_dir).expect("Не могу прочитать директорию");
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();

                if path.is_file() && path.extension().unwrap() == "png" {
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
