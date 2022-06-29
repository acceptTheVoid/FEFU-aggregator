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
    body: String,
}

#[get("/")]
pub async fn index(session: &CookieJar<'_>) -> Template {
    let cookie = session.get_private("username").map(|c| c.value().to_string());
    let session = session.get_private("username").is_some();
    Template::render("index", context! { account: cookie, session, })
}

#[get("/group/<tag>")]
pub async fn group(tag: &str, tags: &State<Tags>, session: &CookieJar<'_>) -> Option<Template> {
    tags.get(tag)?;
    let session = session.get_private("username").is_some();
    Some(Template::render("feed", context! { session }))
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
    let posts: Vec<Send> = posts.items.into_iter().map(|p| Send { body: p.text, }).collect();  

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

#[get("/logout")]
pub async fn logout(session: &CookieJar<'_>) -> Redirect {
    session.remove_private(Cookie::named("username"));
    Redirect::to("/")
}

#[get("/<category>")]
pub async fn news(category: &str, groups: &State<Config>, session: &CookieJar<'_>) -> Option<Template> {
    let session = session.get_private("username").is_some();
    Some(Template::render("category", context! { category: &groups.get(category)?, session, }))
}