# Как пользоваться?
- Для того чтобы начать конвертацию, создайте новый экземпляр структуры с помощью `Compressor::new()`, который принимает два аргумента типа &'static str, первый - входная директория, откуда программа будет брать картинки, второй - куда будут сохраняться уже сжатые картинки. Далее надо просто обратиться к экземпляру структуру и вызвать метод `compress_of_images()` и дождаться полной конвертации. Более подробно ознакомиться с примерами вы можете в документации: клонируйте данный репозиторий, и запустите `cargo doc --open`.

# Что эта программа делает?
- Сжимает все ваши .jpg, .jpeg, .png изображения в формат .webp без потери качества, в общем, картинка сжимается примерно в 1.5 раза, при этом не теряя качества, но иногда дело может доходить до ужатия в 3 раза без потери качества (см. пример в `input_images`, где лежит исходной файл и в `output_images` где лежит сжатый .webp файл)
