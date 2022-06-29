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
#[serde(crate = "rocket::serde")]
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
#[derive(Debug, Queryable, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Post {
    id: i32,
    text: String,
    date: NaiveDateTime,
    author_id: i32,
    thread_id: i32,
    likes: i32,
    dislikes: i32,
}

#[derive(Debug, Insertable, Deserialize, FromForm)]
#[serde(crate = "rocket::serde")]
#[table_name = "posts"]
pub struct NewPost {
    text: String,
    author_id: i32,
    thread_id: i32,
    likes: i32,
    dislikes: i32,
}
