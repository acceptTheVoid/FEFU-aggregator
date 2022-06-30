use rocket::serde::Deserialize;
use rvk::objects::Integer;
use crate::schema::*;

#[derive(Debug, FromForm, Deserialize, Insertable, Queryable, Clone)]
#[table_name = "users"]
pub struct NewUser {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize, FromForm)]
pub struct NewThread {
    pub title: String,
    #[field(validate = neq(Some("null".to_string())))]
    pub group_id: Option<String>,
    pub post_id: Option<Integer>,
    pub text: String,
}

#[derive(Debug, FromForm, Deserialize)]
pub struct NewPost {
    pub text: String,
}
