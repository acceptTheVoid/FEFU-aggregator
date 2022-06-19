use rocket::{State, serde::{json::Json, Deserialize}, http::{CookieJar, Cookie}, response::Redirect, form::Form};
use rocket_dyn_templates::Template;
use rvk::{APIClient, objects::post::Post, methods::wall, Params};
use crate::{db::{connection::DBConn, Config, Tags, NewUser}, hash};

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
struct Response {
    items: Vec<Post>,
}

#[derive(Debug, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Send {
    title: String,
    body: String,
    likes: usize,
    reposts: usize,
}

#[get("/")]
pub async fn index(session: &CookieJar<'_>) -> Template {
    let cookie = session.get("username").map(|c| c.value().to_string());
    Template::render("index", context! { account: cookie, })
}

#[get("/<category>")]
pub async fn news(category: &str, groups: &State<Config>) -> Option<Template> {
    Some(Template::render("category", context! { category: &groups.get(category)?, }))
}

#[get("/group/<tag>")]
pub async fn group(tag: &str, tags: &State<Tags>) -> Option<Template> {
    tags.get(tag)?;
    Some(Template::render("feed", context! {}))
}

#[get("/group/<tag>/get?<offset>")]
pub async fn get_posts(
    tag: &str, 
    offset: usize, 
    api: &State<APIClient>,
    tags: &State<Tags>,
) -> Option<Json<Vec<Send>>> {
    let mut params = Params::new();
    params.insert("owner_id".into(), tags.get(tag)?.to_string());
    params.insert("count".into(), "15".into());
    params.insert("offset".into(), format!("{offset}"));

    let posts = wall::get::<Response>(api, params).await.ok()?;
    let posts: Vec<Send> = posts.items.into_iter().map(|p| Send { title: "Aboba".into(), body: p.text, likes: 1, reposts: 2, }).collect();  

    Some(Json(posts))
}

#[get("/register")]
pub async fn register() -> Template {
    Template::render("login", context! { action: "register" })
}

#[post("/register", data = "<user>")]
pub async fn register_post(
    user: Form<NewUser>,
    db: &State<DBConn>,
    session: &CookieJar<'_>,
) -> Redirect {
    let user = user.into_inner();
    if let Some(_) = db.find_user(&user.username).await {
        return Redirect::to(uri!("/register"));
    }
    db.insert_user(&NewUser { username: user.username.clone(), password: hash(&user.password) }).await;
    session.add_private(Cookie::new("username", user.username));

    Redirect::to(uri!("/"))
}

#[get("/login")]
pub async fn login() -> Template {
    Template::render("login", context! { action: "login" })
}

#[post("/login", data = "<user>")]
pub async fn login_post(
    user: Form<NewUser>,
    db: &State<DBConn>,
    session: &CookieJar<'_>,
) -> Redirect {
    let user = user.into_inner();

    if let Some(id) = db.find_user(&user.username).await {
        if db.check_hash(id, &user.password).await {
            match session.get_private("username") {
                Some(mut c) => c.set_value(&user.username),
                None => session.add_private(Cookie::new("username", user.username)),
            }
            return Redirect::to(uri!("/"));
        }
    }

    Redirect::to(uri!("/login"))
}
