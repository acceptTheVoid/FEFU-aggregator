use rocket::{fs::FileServer, figment::Figment, Config};
use rvk::APIClient;
use fefu_aggregator::keys::{get_key, get_secret_key};
use fefu_aggregator::routes::*;

#[macro_use]
extern crate rocket;

use rocket_dyn_templates::Template;

#[launch]
fn rocket() -> _ {
    let vk_key = get_key();
    let secret_key = get_secret_key();
    let api = APIClient::new(vk_key);

    let figment = Figment::from(Config::default())
        .merge(("secret_key", secret_key));

    rocket::custom(figment)
        .manage(api)
        .attach(Template::fairing())
        .mount("/static", FileServer::from("static/"))
        .mount("/", routes![index])
}
