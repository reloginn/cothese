# Как пользоваться?
- Для того чтобы начать работу, создайте новый экземпляр структуры Compressor, внутри которого помести path с картинками и path куда будут сохраняться сжатые картинки:
```
let compressor = Compressor::new("./input_images/", "./output_images/").unwrap()
  .from().webp();
compressor.compress();
```
- В данном примере я создаю экземпляр структуры, далее с помощью метода from() выбираю нужный тип сжатия, в моём случае webp, и с помощью метода compress() запускаю сжатие.

# Что эта программа делает?
- Сжимает все ваши .jpg, .jpeg, .png изображения в формат .webp без потери качества, в общем, картинка сжимается примерно в 1.5 раза, при этом не теряя качества, но иногда дело может доходить до ужатия в 3 раза без потери качества (см. пример в `input_images`, где лежит исходной файл и в `output_images` где лежит сжатый .webp файл)

# Цели
- [ ] Перевести всю библиотеку на async
