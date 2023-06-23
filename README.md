# Как пользоваться?
- Для начала, нам нужно клонировать проект и выполнить cd в него:
```shell
git clone https://github.com/reloginn/image-compressor.git
cd ./image-compressor/
```
- После этого вам потребуется установить **Rust** (если у вас его нет), все инструкции по установке есть тут: https://www.rust-lang.org/tools/install
- После установки **Rust** нам нужно скомпилировать программу указав все нужные аргументы:
```shell
cargo run --release -- <INPUT_PATH> <OUTPUT_PATH> <TYPE: all or png or webp> <LOGS: true or false>
```
- Где `INPUT_PATH` это папка, где лежат изображения которые надо сжать, `OUTPUT_PATH` это папка куда программа будет ложить сжатые изображения (результат), `TYPE` это тип сжатия (**all:** .jpg, .jpeg => .webp, .png => .png, **webp:** .jpg, .jpeg, .png => .webp, **png:** .png => .png) и `LOGS` это параметр, отвечающий за логирование.

# Требования
- **Rust 1.65+**
