use rocket::serde::Deserialize;
use rvk::objects::Integer;
use std::collections::HashMap;
use crate::schema::*;

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct Group {
    pub name: String,
    pub href: String,
    pub id: Integer,
    pub tag: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, Queryable)]
#[serde(crate = "rocket::serde")]
pub struct CachedPost {
    pub id: Integer,
    pub text: String,
    pub group_id: Integer,
}

pub type Tags = HashMap<String, Integer>;
pub type Config = HashMap<String, Vec<Group>>;

#[derive(Debug, FromForm, Deserialize, Insertable, Queryable, Clone)]
#[serde(crate = "rocket::serde")]
#[table_name = "users"]
pub struct NewUser {
    pub username: String,
    pub password: String,
}

#[derive(Debug, FromForm, Deserialize, Insertable, Queryable, Clone)]
#[serde(crate = "rocket::serde")]
#[table_name = "users"]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
}