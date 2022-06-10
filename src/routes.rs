use rocket::State;
use rocket_dyn_templates::Template;
use rvk::APIClient;

#[get("/")]
pub async fn index(_api: &State<APIClient>) -> Template {
    Template::render("index", context! {})
}