use db::NewUser;
use rocket::http::{CookieJar, Cookie};

pub mod init;
pub mod routes;
pub mod db;
pub mod schema;

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate rocket_dyn_templates;

#[macro_use]
extern crate diesel;

pub fn hash(password: &str) -> String {
    use sha2::{Sha512, Digest};
    let mut hasher = Sha512::new();
    hasher.update(password.as_bytes());
    let res = hasher.finalize();
    // println!("{:?}", res);
    let res = res.to_vec();
    let res = String::from_utf8_lossy(&res);
    // println!("{}", res);

    res.to_string()
}
