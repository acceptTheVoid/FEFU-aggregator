#![allow(dead_code)]

use std::{fs::File, io::{self, BufRead, BufReader, Write}, thread::sleep, time::Duration};
use fefu_aggregator::keys::get_key;
use rocket::serde::Deserialize;
use rvk::{APIClient, Params, methods::wall, objects::post::Post};

struct Info {
    succesful: usize,
    errors: usize,
    reasons: Vec<(String, usize, usize)>,
}

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
struct Resp {
    count: usize,
    items: Vec<Post>,
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let key = get_key();
    let api = APIClient::new(key);

    let mut log = File::create("group_log.txt")?;

    let group_list = File::open("group_list.txt")?;
    let reader = BufReader::new(group_list);
    
    let lines = reader.lines();
    let lines: Vec<String> = lines
        .filter(|s| !s.as_ref().unwrap().is_empty())
        .map(|s| s.unwrap())
        .collect();

    for i in (0..lines.len()).step_by(2) {
        let name = &lines[i];
        let href = &lines[i + 1];

        let res = search_in(&api, href).await;
        println!("Завершена проверка {name}");
        let res = format!(
            "Результаты проверки {name}:\nПройдено постов: {}\nИз них успешно: {}\nИз них ошибочно: {}\nПричины: {:#?}\n", 
            res.succesful + res.errors, res.succesful, res.errors, res.reasons,
        );
        log.write_all(res.as_bytes())?;
    }

    Ok(())
}

async fn search_in(api: &APIClient, href: &str) -> Info {
    let mut href = href.split_ascii_whitespace();
    let id = href.nth(1).unwrap();
    let (mut succesful, mut errors) = (0, 0);
    let mut reasons = Vec::new();

    loop {
        let mut params = Params::new();
        params.insert("owner_id".into(), id.into());
        params.insert("count".into(), "100".into());
        params.insert("offset".into(), format!("{}", succesful + errors));

        sleep(Duration::from_millis(200));
        let res = wall::get::<Resp>(api, params).await;
        match res {
            Ok(posts) => {
                succesful += posts.items.len();
                println!("Пройдено {} постов", succesful + errors);
                if posts.items.is_empty() { break; }
            },
            Err(_) => {
                println!("Встречена ошибка!");
                for _ in 0..100 {
                    sleep(Duration::from_millis(200));
                    let mut params = Params::new();
                    params.insert("owner_id".into(), id.into());
                    params.insert("count".into(), "1".into());
                    params.insert("offset".into(), format!("{}", succesful + errors));

                    let res = wall::get::<Resp>(api, params).await;
                    match res {
                        Ok(posts) => {
                            succesful += 1;
                            if posts.items.is_empty() { break; }
                        },
                        Err(e) => {
                            errors += 1;
                            println!("На позиции {} встречена ошибка: {e}", succesful + errors);
                            reasons.push((e.to_string(), succesful + errors, errors))
                        }
                    }
                }
            }
        }
    }

    Info {
        succesful,
        errors,
        reasons,
    }
}
