use crate::{trash::quantize_png, Dir, IterMutex};
use std::fs;

#[derive(Debug)]
pub struct Png {
    pub(crate) input_dir: Dir,
    pub(crate) output_dir: Dir,
    pub(crate) _logs: bool,
    pub(crate) iter: IterMutex,
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
                    )
                }
            }
        }
    }
}
