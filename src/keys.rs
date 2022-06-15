use std::{fs::read_to_string, process::exit};

pub fn get_secret_key() -> String {
    let mut key_dir = dirs::config_dir().unwrap_or_else(|| {
        eprintln!("Не удалось найти директорию конфига");
        exit(1);
    });
    key_dir.push("secret_key");

    let key = read_to_string(key_dir).unwrap_or_else(|e| {
        eprintln!("Не удалось прочитать файл: {e}");
        exit(1);
    });

    key
}

pub fn get_key() -> String {
    let mut key_dir = dirs::config_dir().unwrap_or_else(|| {
        eprintln!("Не удалось найти директорию конфига");
        exit(1);
    });
    key_dir.push("key");

    let key = read_to_string(key_dir).unwrap_or_else(|e| {
        eprintln!("Не удалось прочитать файл: {e}");
        exit(1);
    });

    key
}
