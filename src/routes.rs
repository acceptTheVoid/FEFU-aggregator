use chrono::Local;
use rocket::{State, http::{CookieJar, Cookie, Status}, response::Redirect, form::Form, serde::json::Json};
use serde::{Deserialize, Serialize};
use rocket_dyn_templates::Template;
use rvk::{APIClient, objects::{post::Post, Integer}, methods::wall, Params};
use crate::{db::{connection::DBConn, Config, Tags, models::{ff_models::{NewUser, self}, db_models::{self, Thread}}}, hash};

#[derive(Debug, Deserialize)]
struct Response {
    items: Vec<Post>,
}

#[derive(Debug, Serialize)]
pub struct Send {
    body: String,
    id: Integer,
    threads: Vec<Thread>,
}

type PostsResult = Result<Json<Vec<Send>>, Json<String>>;

#[get("/")]
pub async fn index(session: &CookieJar<'_>) -> Template {
    let account = session.get_private("username")
        .map(|c| c.value().to_string());
    let session = account.is_some();

    Template::render("index", context! { account, session })
}

#[get("/group/<tag>")]
pub async fn group(tag: &str, tags: &State<Tags>, session: &CookieJar<'_>) -> Option<Template> {
    tags.get(tag)?;
    let session = session.get_private("user_id").is_some();
    Some(Template::render("feed", context! { tag, session }))
}

#[get("/group/<tag>/get?<offset>")]
pub async fn get_posts(
    tag: &str, 
    offset: usize, 
    api: &State<APIClient>,
    tags: &State<Tags>,
    db: &State<DBConn>,
) -> PostsResult {
    let mut params = Params::new();
    let tag = match tags.get(tag) {
        Some(t) => *t,
        None => return Err(Json(format!("{tag} does not exist"))),
    };

    params.insert("owner_id".into(), tag.to_string());
    params.insert("count".into(), "15".into());
    params.insert("offset".into(), format!("{offset}"));

    let posts: Response = match wall::get(api, params).await {
        Ok(r) => r,
        Err(e) => return Err(Json(e.to_string())),
    };

    let mut threads = Vec::with_capacity(15);
    for i in &posts.items {
        let t = match db.get_threads_from(tag, i.id).await {
            Ok(v) => v,
            Err(e) => return Err(Json(e.to_string())),
        };
        threads.push(t);
    }

    let posts: Vec<Send> = posts.items.into_iter()
        .zip(threads.into_iter()).map(|(p, threads)| Send { 
            body: p.text, 
            id: p.id, 
            threads,
        }).collect();  

    Ok(Json(posts))
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
) -> Result<Redirect, Status> {
    let user = user.into_inner();
    if let Some(_) = db.find_user(&user.username).await {
        return Err(Status::Conflict);
    }

    match db.insert_user(&NewUser { 
        username: user.username.clone(), 
        password: hash(&user.password) 
    }).await {
        Err(_) => return Err(Status::InternalServerError),
        _ => (),
    };
    let id = db.find_user(&user.username).await.unwrap();
    
    session.add_private(Cookie::new("username", user.username));
    session.add_private(Cookie::new("user_id", id.to_string()));

    Ok(Redirect::to(uri!("/")))
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
) -> Result<Redirect, Status> {
    let user = user.into_inner();

    if let Some(id) = db.find_user(&user.username).await {
        match db.check_hash(id, &user.password).await {
            Ok(res) => {
                if res {
                    session.add_private(Cookie::new("user_id", id.to_string()));
                    session.add_private(Cookie::new("username", user.username));
                    return Ok(Redirect::to(uri!("/")));
                }
            },
            _ => return Err(Status::InternalServerError),
        }
    }

    Ok(Redirect::to(uri!("/login")))
}

#[get("/logout")]
pub async fn logout(session: &CookieJar<'_>) -> Redirect {
    session.remove_private(Cookie::named("username"));
    session.remove_private(Cookie::named("user_id"));
    Redirect::to("/")
}

#[get("/<category>")]
pub async fn news(
    category: &str, 
    groups: &State<Config>, 
    session: &CookieJar<'_>
) -> Option<Template> {
    let session = session.get_private("user_id").is_some();
    Some(Template::render("category", context! { category: &groups.get(category)?, session }))
}

#[get("/threads")]
pub async fn threads(db: &State<DBConn>, session: &CookieJar<'_>) -> Result<Template, Status> {
    let threads = match db.get_threads().await {
        Ok(t) => t,
        _ => return Err(Status::InternalServerError),
    };
    let session = session.get_private("user_id").is_some();

    Ok(Template::render("threads", context!{ threads, session }))
}

#[get("/new_thread")]
pub async fn new_thread(session: &CookieJar<'_>) -> Result<Template, Status> {
    let session = session.get_private("user_id").is_some();
    if !session {
        return Err(Status::Unauthorized);
    }

    Ok(Template::render("new_thread", context!{ session }))
}

#[post("/new_thread", data = "<thread>")]
pub async fn new_thread_post(
    session: &CookieJar<'_>,
    db: &State<DBConn>,
    thread: Form<ff_models::NewThread>,
    tags: &State<Tags>,
) -> Result<Redirect, Status> {
    match session.get_private("user_id") {
        Some(c) => {
            let id: i32 = c.value().parse().unwrap();
            println!("{:#?}", thread);
            let thread = thread.into_inner();
            let thread = db_models::NewThread {
                title: thread.title,
                group_id: match thread.group_id {
                    Some(n) => tags.get(&n).map(|n| *n),
                    None => None,
                },
                post_id: thread.post_id,
                text: thread.text,
                date: Local::now().naive_local(),
                author_id: id,
            };

            match db.insert_thread(thread).await {
                Err(_) => return Err(Status::InternalServerError),
                _ => (),
            };

            Ok(Redirect::to(uri!("/threads")))
        },
        None => Err(Status::Unauthorized),
    }
}

#[get("/thread/<id>")]
pub async fn thread(
    id: i32, 
    db: &State<DBConn>, 
    session: &CookieJar<'_>
) -> Option<Template> {
    let (thread, posts) = db.get_thread(id).await.ok()?;
    let session = session.get_private("user_id").is_some();

    Some(Template::render("thread", context!{ thread, posts, session }))
}

#[post("/new_post/<thread_id>", data = "<post>")]
pub async fn new_post(
    thread_id: i32, 
    post: Form<ff_models::NewPost>, 
    db: &State<DBConn>, 
    session: &CookieJar<'_>
) -> Result<Redirect, Status> {
    let post = post.into_inner();
    let author_id = match session.get_private("user_id") {
        Some(c) => c.value().parse().unwrap(),
        None => return Err(Status::Unauthorized),
    };

    let post = db_models::NewPost {
        text: post.text,
        date: Local::now().naive_local(),
        author_id,
        thread_id,
    };

    match db.new_post(post).await {
        Err(_) => return Err(Status::InternalServerError),
        _ => (),
    };

    Ok(Redirect::to(uri!(thread(thread_id))))
}

#[get("/get_targets")]
pub async fn get_targets(targets: &State<Config>) -> Json<Config> {
    Json((*targets.inner()).clone())
}

#[derive(Debug, Serialize)]
pub struct LikeInfo {
    pub likes: i32,
    pub dislikes: i32,
}

#[get("/like?<_id>")]
pub async fn like(_id: i32, session: &CookieJar<'_>, _db: &State<DBConn>) -> Result<Json<LikeInfo>, Status> {
    let _user_id: i32 = match session.get_private("user_id") {
        Some(c) => c.value().parse().unwrap(),
        None => return Err(Status::Unauthorized),
    };

    todo!()
}

#[get("/dislike?<_id>")]
pub async fn dislike(_id: i32) -> Result<Json<LikeInfo>, Status> {
    todo!()
}