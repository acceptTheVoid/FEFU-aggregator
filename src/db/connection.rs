use tokio::sync::Mutex;
use diesel::prelude::*;
use std::env;

use super::{NewUser, User};

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

    pub async fn find_user(&self, username: &String) -> Option<i32> {
        use crate::schema::users::{dsl, self};
        let conn = &*self.conn.lock().await;
        let res: Option<i32> = users::table
            .filter(dsl::username.eq(username))
            .select(dsl::id)
            .first(conn).ok();
        
        res
    }

    pub async fn insert_user(&self, user: &NewUser) {
        use crate::schema::users;
        let conn = &*self.conn.lock().await;
        diesel::insert_into(users::table)
            .values(user)
            .execute(conn)
            .unwrap();
    }

    pub async fn check_hash(&self, id: i32, password: &str) -> bool {
        use crate::schema::users;
        let conn = &*self.conn.lock().await;
        let user: User = users::table.find(id).first(conn).unwrap();
        
        user.password == password
    }
}

