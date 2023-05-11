#![allow(unused)]

use std::{path::PathBuf, sync::Arc};

use walkdir::WalkDir;

#[derive(Debug)]
pub struct PNG {
    pub(crate) input_dir: Arc<PathBuf>,
    pub(crate) output_dir: Arc<PathBuf>,
}

impl PNG {
    pub fn compress(&self) {
        println!("Заглушка");
    }
    fn get_all_of_files_in_directory(&self) -> Vec<PathBuf> {
        let mut file_paths: Vec<PathBuf> = Vec::new();

        for entry in WalkDir::new(&*self.input_dir)
            .max_depth(1)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            if entry.file_type().is_file() {
                file_paths.push(entry.path().to_path_buf());
            }
        }
        file_paths
    }
}
