use std::fs::read_to_string;

pub fn get_secret_key() -> String {
    let mut key_dir = dirs::config_dir().expect("Не удалось найти директорию конфига");
    key_dir.push("secret_key");

    let key = read_to_string(key_dir).expect("Не удалось прочитать файл");

    key
}

pub fn get_key() -> String {
    let mut key_dir = dirs::config_dir().expect("Не удалось найти директорию конфига");
    key_dir.push("key");

    let key = read_to_string(key_dir).expect("Не удалось прочитать файл");

    key
}
