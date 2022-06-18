use rocket::{State, serde::{json::Json, Deserialize}};
use rocket_dyn_templates::Template;
use rvk::{APIClient, objects::{Integer, post::Post}, methods::wall, Params};
use crate::db::{connection::DBConn, Config, Tags, CheckResponse};

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
struct Response {
    items: Vec<Post>,
}

#[derive(Debug, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Send {
    text: String,
}

#[get("/")]
pub async fn index() -> Template {
    Template::render("index", context! {})
}

#[get("/<category>")]
pub async fn news(category: &str, groups: &State<Config>) -> Option<Template> {
    Some(Template::render("category", context! { category: &groups.get(category)?, }))
}

#[get("/group/<id>")]
pub async fn group(id: &str, tags: &State<Tags>) -> Option<Template> {
    tags.get(id)?;
    Some(Template::render("feed", context! {}))
}

#[get("/group/<id>/check?<last_post>")]
pub async fn check(
    id: &str,
    last_post: Integer,
    tags: &State<Tags>, 
    db: &State<DBConn>
) -> Json<CheckResponse> {
    use CheckResponse::*;

    let id = match tags.get(id) {
        Some(n) => *n,
        None => return Json(WrongAddress),
    };
    let id = db.check(last_post).await;
    
    let response = match id {
        Some(n) if n == 0 => Complete,
        Some(n) => Missing(n),
        None => Empty,
    };

    Json(response)
}

#[get("/group/<id>/get?<offset>")]
pub async fn get_posts(
    id: &str, 
    offset: usize, 
    api: &State<APIClient>,
    tags: &State<Tags>,
) -> Option<Json<Vec<Send>>> {
    let mut params = Params::new();
    params.insert("owner_id".into(), tags.get(id)?.to_string());
    params.insert("count".into(), "100".into());
    params.insert("offset".into(), format!("{offset}"));

    let posts = wall::get::<Response>(api, params).await.ok()?;
    let posts: Vec<Send> = posts.items.into_iter().map(|p| Send { text: p.text }).collect();  
    
    Some(Json(posts))
}
