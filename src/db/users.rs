use db::Database;
use diesel::prelude::*;
use diesel;
use models::{User,NewUser};
use schema::users::dsl::*;


impl User {
    pub fn all(conn: &Database, limit: i64) -> Vec<User> {
         users.limit(limit)
            .load::<User>(&conn.0)
            .expect("Error loading users")
            .into_iter()
            .collect()
    }

    pub fn get(conn: &Database, user_id: i32) -> Option<User> {
        let result = users.filter(id.eq(user_id))
            .first::<User>(&conn.0);

        match result {
            Ok(p) => Some(p),
            Err(diesel::NotFound) => None,
            Err(_) => panic!("Failed to retreive one User"),
        }
    }

    pub fn get_by_email_or_name(conn: &Database, user_email: &str, user_name: &str) -> Option<User> {
        let result = users.filter(email.eq(user_email).or(name.eq(user_name)))
            .first::<User>(&conn.0);

        match result {
            Ok(p) => Some(p),
            Err(diesel::NotFound) => None,
            Err(e) => panic!("Failed to retreive User by email: {}", e),
        }
    }

    pub fn get_by_email(conn: &Database, user_email: &str) -> Option<User> {
        let result = users.filter(email.eq(user_email))
            .first::<User>(&conn.0);

        match result {
            Ok(p) => Some(p),
            Err(diesel::NotFound) => None,
            Err(e) => panic!("Failed to retreive User by email: {}", e),
        }
    }

    pub fn insert(conn: &Database, user: &NewUser) -> User {
        diesel::insert_into(users)
            .values(user)
            .get_result::<User>(&conn.0)
            .expect("Failed to insert user")
    }

    pub fn count(conn: &Database) -> i64 {
         users.count()
            .get_result(&conn.0)
            .expect("Error loading users")
    }
}
