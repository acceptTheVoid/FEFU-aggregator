use std::{io::{Result, Write}, fs::File, env};

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("Недостаточно аргументов! Вам нужно указать ключ для начала");
    }

    let key = args[1].trim();

    let mut key_dir = dirs::config_dir().expect("Не удалось найти папку конфига");
    key_dir.push("key");

    let mut f = File::create(key_dir)?;
    f.write_all(key.as_bytes())?;

    Ok(())
}
