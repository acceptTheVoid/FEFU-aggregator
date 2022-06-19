use rocket::{fs::FileServer, figment::Figment, Config};
use rvk::APIClient;
use fefu_aggregator::init::{get_key, get_secret_key, get_groups};
use fefu_aggregator::routes::*;
use fefu_aggregator::db::connection::DBConn;

#[macro_use]
extern crate rocket;

use rocket_dyn_templates::Template;

#[launch]
fn rocket() -> _ {
    let vk_key = get_key();
    let secret_key = get_secret_key();
    let (config, tags) = get_groups();
    let api = APIClient::new(vk_key);
    let db = DBConn::new();

    let figment = Figment::from(Config::default())
        .merge(("secret_key", secret_key));

    rocket::custom(figment)
        .manage(api)
        .manage(config)
        .manage(tags)
        .manage(db)
        .attach(Template::fairing())
        .mount("/static", FileServer::from("static/"))
        .mount("/", routes![index, news, get_posts, group, login, register, register_post, login_post])
}
