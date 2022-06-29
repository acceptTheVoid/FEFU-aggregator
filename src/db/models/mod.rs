pub mod db_models;
pub mod ff_models;

use rocket::serde::Deserialize;
use rvk::objects::Integer;
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct Group {
    pub name: String,
    pub href: String,
    pub id: Integer,
    pub tag: String,
}

pub type Tags = HashMap<String, Integer>;
pub type Config = HashMap<String, Vec<Group>>;
