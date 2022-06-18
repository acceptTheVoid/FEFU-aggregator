use tokio::sync::Mutex;
use diesel::{prelude::*, dsl::sql};
use rvk::objects::Integer;
use std::env;

use crate::db::CachedPost;

pub struct DBConn {
    pub conn: Mutex<MysqlConnection>,
}

impl DBConn {
    pub fn new() -> Self {
        dotenv::dotenv().ok();
        let db_uri = env::var("DATABASE_URL")
            .expect("DATABASE_URL variable must be set");
    
        let conn = MysqlConnection::establish(&db_uri)
            .expect(&format!("Error connection to {db_uri}"));
        let conn = Mutex::new(conn);

        Self { conn }
    }

    pub async fn check(&self, last_id: Integer) -> Option<Integer> {
        use crate::schema::post::dsl::*;
        let conn = &*self.conn.lock().await;
        let max_id = post.select(sql("MAX(id)")).first(conn);

        max_id.ok().map(|db_id: Integer| (db_id - last_id).abs())
    }
}
