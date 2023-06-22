# Как пользоваться?
- Вы можете просто написать:
```shell
git clone https://github.com/reloginn/image-compressor.git
cd ./image-compressor/
cargo run --release -- <INPUT_PATH> <OUTPUT_PATH> <TYPE: all or webp or png> <LOGS: true or false>
```
- Где `INPUT_PATH` это папка, где лежат изображения которые надо сжать, `OUTPUT_PATH` это папка куда программа будет ложить сжатые изображения (результат), `TYPE` это тип сжатия (all: .jpg, .jpeg => .webp, .png => .png, webp: .jpg, .jpeg, .png => .webp, png: .png => .png) и `LOGS` это параметр, отвечающий за логирование.

# Требования
- Rust 1.65+
