use crate::{
    trash::{compress_jpeg_to_webp, quantize_a_png_image},
    Dir, IterMutex,
};
use std::fs;

#[derive(Debug)]
pub struct All {
    pub input_dir: Dir,
    pub output_dir: Dir,
    pub _logs: bool,
    pub iter: IterMutex,
}

impl All {
    pub fn compress(self) {
        let entries =
            fs::read_dir(self.input_dir.lock().unwrap().as_ref()).expect("Cannot read a directory");
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() {
                match path.extension().unwrap().to_str().unwrap() {
                    "jpeg" | "jpg" => {
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
                    "png" => {
                        *self.iter.lock().unwrap() += 1;
                        quantize_a_png_image(
                            path,
                            self.output_dir
                                .lock()
                                .unwrap()
                                .as_ref()
                                .join(format!("{}", *self.iter.lock().unwrap()))
                                .with_extension("png"),
                            self._logs,
                        )
                    }
                    _ => (),
                }
            }
        }
    }
}
