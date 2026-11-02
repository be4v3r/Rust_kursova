use std::{env, fs, process};

fn main() {
    let mut args = env::args_os();
    let program = args.next().unwrap_or_default();
    let path = match args.next() {
        Some(path) => path,
        None => {
            eprintln!("Помилка: вкажіть шлях до текстового файлу.\nВикористання: {} <файл>", program.to_string_lossy());
            process::exit(2);
        }
    };
    if args.next().is_some() {
        eprintln!("Помилка: очікується лише один аргумент — шлях до файлу.\nВикористання: {} <файл>", program.to_string_lossy());
        process::exit(2);
    }
    let text = match fs::read_to_string(&path) {
        Ok(text) => text,
        Err(error) => {
            eprintln!("Не вдалося прочитати файл '{}': {}", path.to_string_lossy(), error);
            eprintln!("Переконайтеся, що шлях правильний, файл доступний, а текст збережено у UTF-8.");
            process::exit(1);
        }
    };
    let count = text.split_whitespace().count();
    println!("Кількість слів: {count}");
}
