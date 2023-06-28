pub const HELP: &str = "
Image Compressor

Использование: image-compressor [FLAGS]

Опции:
    -h, --help          Выводит помощь
    -v, --version       Выводит версию

Флаги:
    --input [Директория]         Входная директория с изображениями (ОБЯЗАТЕЛЬНЫЙ ПАРАМЕТР)
    --output [Директория]        Директория куда будут сохраняться готовые изображения (ОБЯЗАТЕЛЬНЫЙ ПАРАМЕТР)
    --type [Тип сжатия]          all или jpg-webp (НЕОБЯЗАТЕЛЬНО)
    --logs [bool]                true или false, отвечает за логирование действий (НЕОБЯЗАТЕЛЬНО)

Дополнительная информация:
    --type all                   Сжимает файлы по такой схеме: .png, .jpeg, .jpg => .webp
    --type jpg-webp              Сжимает файлы по такой схеме: .jpg, .jpeg => .webp               

";
pub const VERSION: &str = "
Image Compressor версии 0.0.1
";