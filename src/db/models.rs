use rocket::serde::Deserialize;
use rvk::objects::Integer;
use std::collections::HashMap;
use crate::schema::post;

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct Group {
    pub name: String,
    pub href: String,
    pub id: Integer,
    pub tag: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, Queryable, Insertable)]
#[serde(crate = "rocket::serde")]
#[table_name = "post"]
pub struct CachedPost {
    pub id: Integer,
    pub text: String,
}

pub type Tags = HashMap<String, Integer>;
pub type Config = HashMap<String, Vec<Group>>;

#[derive(Debug, Serialize, Clone, Copy)]
#[serde(crate = "rocket::serde")]
#[serde(tag = "status", content = "amount")]
pub enum CheckResponse {
    Complete,
    Missing(Integer),
    Empty,
    WrongAddress,
}
