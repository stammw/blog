use db::Database;
use diesel::prelude::*;
use diesel;
use models::{User,NewUser};
use rocket::Request;
use rocket::request::{self, FromRequest};
use schema::users::dsl::*;

pub type UserRepo = Box<UserRepoTrait + Send>;

pub trait UserRepoTrait {
    fn all(&self, limit: i64) -> Vec<User>;
    fn get(&self, user_id: i32) -> Option<User>;
    fn get_by_email(&self, name: &str) -> Option<User>;
    fn get_by_email_or_name(&self, user_email: &str, name: &str) -> Option<User>;
    fn insert(&self, user: &NewUser) -> User;
    fn count(&self) -> i64;
}

pub type UserRepoFactory = fn(db: Database) -> UserRepo;

pub struct UserRepoImpl {
  pub db: Database,
}

pub fn factory(db: Database) -> UserRepo {
    Box::new(UserRepoImpl { db: db }) as UserRepo   
}

impl UserRepoTrait for UserRepoImpl {
    fn all(&self, limit: i64) -> Vec<User> {
         users.limit(limit)
            .load::<User>(&*self.db)
            .expect("Error loading users")
            .into_iter()
            .collect()
    }

    fn get(&self, user_id: i32) -> Option<User> {
        let result = users.filter(id.eq(user_id))
            .first::<User>(&*self.db);

        match result {
            Ok(p) => Some(p),
            Err(diesel::NotFound) => None,
            Err(_) => panic!("Failed to retreive one User"),
        }
    }

    fn get_by_email_or_name(&self, user_email: &str, user_name: &str) -> Option<User> {
        let result = users.filter(email.eq(user_email).or(name.eq(user_name)))
            .first::<User>(&*self.db);

        match result {
            Ok(p) => Some(p),
            Err(diesel::NotFound) => None,
            Err(e) => panic!("Failed to retreive User by email: {}", e),
        }
    }

    fn get_by_email(&self, user_email: &str) -> Option<User> {
        let result = users.filter(email.eq(user_email))
            .first::<User>(&*self.db);

        match result {
            Ok(p) => Some(p),
            Err(diesel::NotFound) => None,
            Err(e) => panic!("Failed to retreive User by email: {}", e),
        }
    }

    fn insert(&self, user: &NewUser) -> User {
        diesel::insert_into(users)
            .values(user)
            .get_result::<User>(&*self.db)
            .expect("Failed to insert user")
    }

    fn count(&self) -> i64 {
         users.count()
            .get_result(&*self.db)
            .expect("Error loading users")
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for UserRepo {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<UserRepo, ()> {
        request.guard::<Database>()
            .map(|db| factory(db))
    }
}

