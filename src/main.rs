use rocket::fs::FileServer;
use rvk::APIClient;
use vk_api::get_key;
use routes::*;

pub mod vk_api;
pub mod routes;

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate rocket_dyn_templates;

use rocket_dyn_templates::Template;

#[launch]
fn rocket() -> _ {
    let key = get_key();
    let api = APIClient::new(key);

    rocket::build()
        .manage(api)
        .attach(Template::fairing())
        .mount("/static", FileServer::from("static/"))
        .mount("/", routes![index])
}
