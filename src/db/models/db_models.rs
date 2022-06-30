use chrono::NaiveDateTime;
use rocket::serde::Deserialize;
use rvk::objects::Integer;
use crate::schema::*;

#[derive(Debug, FromForm, Deserialize, Queryable, Clone)]
#[serde(crate = "rocket::serde")]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
}

#[derive(Debug, Insertable)]
#[table_name = "threads"]
pub struct NewThread {
    pub title: String,
    pub group_id: Option<Integer>,
    pub post_id: Option<Integer>,
    pub text: String,
    pub date: NaiveDateTime,
    pub author_id: i32,
}

#[derive(Debug, Queryable, Serialize)]
pub struct Thread {
    pub id: i32,
    pub title: String,
    pub group_id: Option<Integer>,
    pub post_id: Option<Integer>,
    pub text: String,
    pub date: NaiveDateTime,
    pub author_id: i32,
    pub likes: i32,
    pub dislikes: i32,
}

#[allow(unused)]
#[derive(Debug, Queryable, Serialize)]
pub struct Post {
    pub id: i32,
    pub text: String,
    pub date: NaiveDateTime,
    pub author_id: i32,
    pub thread_id: i32,
    pub likes: i32,
    pub dislikes: i32,
}

#[derive(Debug, Insertable, Deserialize)]
#[table_name = "posts"]
pub struct NewPost {
    pub text: String,
    pub date: NaiveDateTime,
    pub author_id: i32,
    pub thread_id: i32,
}
