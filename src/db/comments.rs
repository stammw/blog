use db::Database;
use diesel::prelude::*;
use diesel;
use models::{Comment, NewComment};
use schema::comments::dsl::*;

impl Comment {
    pub fn insert(conn: &Database, comment: &NewComment) -> Comment {
        diesel::insert_into(comments)
            .values(comment)
            .get_result::<Comment>(&conn.0)
            .expect("Failed to insert comment")
    }
}
