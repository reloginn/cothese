pub const HELP_RU: &str = "

cothese 0.0.5

Использование: cothese --input [Директория] --output [Директория] [ДРУГИЕ]

Флаги:
    -h, --help                   Выводит помощь
    -v, --version                Выводит версию

Опции:
    --input [Директория]         Входная директория с изображениями (ОБЯЗАТЕЛЬНЫЙ ПАРАМЕТР)
    --output [Директория]        Директория куда будут сохраняться готовые изображения (ОБЯЗАТЕЛЬНЫЙ ПАРАМЕТР)
    --logs [bool]                true или false, отвечает за логирование действий (НЕОБЯЗАТЕЛЬНО)
    --threads [Количество]       Указывает количество воркеров в работе, значение по умолчанию: 8
    --quality [10.0-100.0]       Качество сжатого изображения, значение по умолчанию: 90.0. Влияет на скорость сжатия и качество изображения.
    --lang=[lang]                Отвечает за язык. По умолчанию язык -- английский, также доступен русский (ru), чтобы использовать просто добавьте --lang=ru.
";
pub const HELP_EN: &str = "

cothese 0.0.5

Usage: cothese --input [Directory] --output [Directory] [OTHER]

Flags:
    -h, --help                  Outputs help
    -v, --version               Outputs version

Options:
    --input [Directory]         Input directory with images (MUST BE PARAMETER)
    --output [Directory]        The directory where your finalized images will be saved (MUST BE PARAMETER)
    --logs [true/false]         Responsible for logging actions (MUST)
    --threads [count]           Indicates the number of vorkers in operation, default value: 8
    --quality [10.0-100.0]      Quality of the compressed image, default value: 90.0. Affects the compression speed and quality of the image.
    --lang=[lang]               English is the default and Russian (ru) is also supported, to use Russian just write --lang=ru.";
pub const VERSION_RU: &str = "
cothese версии 0.0.5
";
pub const VERSION_EN: &str = "
cothese version 0.0.5
";

pub const BOLD: &str = "\x1b[1m";
pub const DEFAULT: &str = "\x1b[0m";
pub const RED: &str = "\x1b[31m";
pub const YELLOW: &str = "\x1b[33m";
pub const BLUE: &str = "\x1b[34m";
