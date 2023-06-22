use crate::{trash::quantize_png, Dir, IterMutex};
use std::fs;

#[derive(Debug)]
pub struct Png {
    pub input_dir: Dir,
    pub output_dir: Dir,
    pub _logs: bool,
    pub iter: IterMutex,
}

impl Png {
    pub fn compress(&self) {
        let entries = fs::read_dir(self.input_dir.lock().unwrap().as_ref())
            .expect("Не могу прочитать директорию");
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() {
                if let "png" = path.extension().unwrap().to_str().unwrap() {
                    *self.iter.lock().unwrap() += 1;
                    quantize_png(
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
            }
        }
    }
}
