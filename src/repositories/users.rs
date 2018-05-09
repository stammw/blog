use db::Database;
use diesel::prelude::*;
use diesel;
use models::{User,NewUser};
use rocket::Request;
use rocket::request::{FromRequest, Outcome};
use schema::users::dsl::*;

pub struct UserRepositoryImpl(Database);

pub trait UserRepository {
    fn all(&self, limit: i64) -> Vec<User>;
    fn get(&self, user_id: i32) -> Option<User>;
    fn insert(&self, user: &NewUser) -> User;
    fn count(&self) -> i64;
}

impl UserRepository for UserRepositoryImpl {
    fn all(&self, limit: i64) -> Vec<User> {
         users.limit(limit)
            .load::<User>(&*self.0)
            .expect("Error loading users")
            .into_iter()
            .collect()
    }

    fn get(&self, user_id: i32) -> Option<User> {
        let result = users.filter(id.eq(user_id))
            .first::<User>(&*self.0);

        match result {
            Ok(p) => Some(p),
            Err(diesel::NotFound) => None,
            Err(e) => panic!("Failed to retreive one User"),
        }
    }

    fn insert(&self, user: &NewUser) -> User {
        diesel::insert_into(users)
            .values(user)
            .get_result::<User>(&*self.0)
            .expect("Failed to insert user")
    }

    fn count(&self) -> i64 {
         users.count()
            .get_result(&*self.0)
            .expect("Error loading users")
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for Box<UserRepository> {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> Outcome<Box<UserRepository>, ()> {
        Database::from_request(request)
            .map(|d| Box::new(UserRepositoryImpl(d)) as Box<UserRepository>)
    }
}
