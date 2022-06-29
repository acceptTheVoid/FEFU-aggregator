use rocket::serde::Deserialize;
use rvk::objects::Integer;
use crate::schema::*;

#[derive(Debug, FromForm, Deserialize, Insertable, Queryable, Clone)]
#[serde(crate = "rocket::serde")]
#[table_name = "users"]
pub struct NewUser {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize, FromForm)]
#[serde(crate = "rocket::serde")]
pub struct NewThread {
    pub title: String,
    pub group_id: Option<Integer>,
    pub post_id: Option<Integer>,
    pub text: String,
}
