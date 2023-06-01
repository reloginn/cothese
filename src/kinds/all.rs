use crate::{
    trash::{compress_jpeg_to_webp, quantize_png},
    Dir, IterMutex,
};
use std::fs;

#[derive(Debug)]
pub struct All {
    pub(crate) input_dir: Dir,
    pub(crate) output_dir: Dir,
    pub(crate) _logs: bool,
    pub(crate) iter: IterMutex,
}

impl All {
    pub fn compress(&self) {
        let entries = fs::read_dir(self.input_dir.lock().unwrap().as_ref())
            .expect("Не могу прочитать директорию");
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
                            self._logs
                        )
                    }
                    "png" => {
                        *self.iter.lock().unwrap() += 1;
                        quantize_png(
                            path,
                            self.output_dir
                                .lock()
                                .unwrap()
                                .as_ref()
                                .join(format!("{}", *self.iter.lock().unwrap()))
                                .with_extension("png"),
                            self._logs
                        )
                    }
                    _ => (),
                }
            }
        }
    }
}
