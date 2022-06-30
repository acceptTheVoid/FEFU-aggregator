use rvk::objects::Integer;
use tokio::sync::Mutex;
use diesel::prelude::*;
use std::env;

use super::{models::{db_models::User, ff_models::NewUser}, db_models::{Thread, NewThread, Post, NewPost}};

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

    pub async fn find_user(&self, username: &str) -> Option<i32> {
        use crate::schema::users::{dsl, self};
        let conn = &*self.conn.lock().await;
        let res: Option<i32> = users::table
            .filter(dsl::username.eq(username))
            .select(dsl::id)
            .first(conn).ok();
        
        res
    }

    pub async fn insert_user(&self, user: &NewUser) -> QueryResult<()> {
        use crate::schema::users;
        let conn = &*self.conn.lock().await;

        diesel::insert_into(users::table)
            .values(user)
            .execute(conn)?;

        Ok(())
    }

    pub async fn check_hash(&self, id: i32, password: &str) -> QueryResult<bool> {
        use crate::schema::users;
        let conn = &*self.conn.lock().await;
        let user: User = users::table.find(id).first(conn)?;

        Ok(user.password == password)
    }

    pub async fn get_threads(&self) -> QueryResult<Vec<(Thread, String)>> {
        use crate::schema::{threads, users};
        let conn = &*self.conn.lock().await;
        
        threads::table.inner_join(users::table)
            .order(threads::id.desc())
            .select((threads::all_columns, users::username))
            .load(conn)
    }

    pub async fn insert_thread(&self, thread: NewThread) -> QueryResult<()> {
        use crate::schema::threads;
        let conn = &*self.conn.lock().await;

        diesel::insert_into(threads::table)
            .values(thread).execute(conn)?;

        Ok(())
    }

    pub async fn get_thread(&self, id: i32) -> QueryResult<(Thread, Vec<Post>)> {
        use crate::schema::{threads, posts};
        let conn = &*self.conn.lock().await;

        let thread: Thread = threads::table.find(id)
            .select(threads::all_columns).first(conn)?;

        let posts: Vec<Post> = posts::table.filter(posts::thread_id.eq(id)).load(conn)?;

        Ok((thread, posts))
    }

    pub async fn new_post(&self, post: NewPost) -> QueryResult<()> {
        use crate::schema::posts;
        let conn = &*self.conn.lock().await;
        
        diesel::insert_into(posts::table).values(post).execute(conn)?;

        Ok(())
    }

    pub async fn get_threads_from(&self, group_id: Integer, post_id: Integer) -> QueryResult<Vec<Thread>> {
        use crate::schema::threads;
        let conn = &*self.conn.lock().await;
        
        threads::table.filter(threads::group_id.eq(group_id))
            .filter(threads::post_id.eq(post_id))
            .select(threads::all_columns)
            .load(conn)
    }
}
