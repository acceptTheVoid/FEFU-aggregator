use std::{fs::read_to_string, process::exit};
use crate::db::{Config, Tags};

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


pub fn get_groups() -> (Config, Tags) {
    let targets = read_to_string("targets.json").unwrap_or_else(|e| {
        eprintln!("Не удалось прочитать файл: {e}");
        exit(1);
    });

    let config: Config = serde_json::from_str(&targets).unwrap_or_else(|e| {
        eprintln!("Не удалось десериализовать данные из файла: {e}");
        exit(1);
    });

    let mut tags = Tags::new();
    for (_, v) in &config {
        for it in v {
            tags.insert(it.tag.clone(), it.id);
        }
    }

    (config, tags)
}
