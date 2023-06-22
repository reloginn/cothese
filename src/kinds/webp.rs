use crate::{trash::compress_jpeg_to_webp, Dir, IterMutex};
use std::fs;

#[derive(Debug)]
pub struct WebP {
    pub input_dir: Dir,
    pub output_dir: Dir,
    pub _logs: bool,
    pub iter: IterMutex,
}

impl WebP {
    pub fn compress(&self) {
        let entries = fs::read_dir(self.input_dir.lock().unwrap().as_ref())
            .expect("Не могу прочитать директорию");
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() {
                if let "jpg" | "jpeg" | "png" = path.extension().unwrap().to_str().unwrap() {
                    *self.iter.lock().unwrap() += 1;
                    compress_jpeg_to_webp(
                        path,
                        self.output_dir
                            .lock()
                            .unwrap()
                            .as_ref()
                            .join(format!("{}", *self.iter.lock().unwrap()))
                            .with_extension("webp"),
                        self._logs,
                    )
                }
            }
        }
    }
}
