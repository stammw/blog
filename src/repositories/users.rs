use db::Database;
use diesel::prelude::*;
use diesel;
use models::{User,NewUser};
use rocket::{State, Request};
use rocket::request::{self, FromRequest};
use schema::users::dsl::*;

pub type UserRepository = Box<UserRepositoryTrait + Send>;

pub trait UserRepositoryTrait {
    fn all(&self, limit: i64) -> Vec<User>;
    fn get(&self, user_id: i32) -> Option<User>;
    fn get_by_name(&self, name: &String) -> Option<User>;
    fn insert(&self, user: &NewUser) -> User;
    fn count(&self) -> i64;
}

pub type UserRepositoryFactory = fn(db: Option<Database>) -> UserRepository;

pub struct UserRepositoryImpl {
  pub db: Database,
}

impl UserRepositoryImpl {
    fn from(db: Option<Database>) -> UserRepository {
        Box::new(Self { db: db.unwrap() }) as UserRepository   
    }
}

impl UserRepositoryTrait for UserRepositoryImpl {
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

    fn get_by_name(&self, user_name: &String) -> Option<User> {
        let result = users.filter(name.eq(user_name))
            .first::<User>(&*self.db);

        match result {
            Ok(p) => Some(p),
            Err(diesel::NotFound) => None,
            Err(_) => panic!("Failed to retreive one User"),
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

impl<'a, 'r> FromRequest<'a, 'r> for UserRepository {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<UserRepository, ()> {
        request.guard::<State<UserRepositoryFactory>>()
            .map(|fact| fact(request.guard::<Database>().succeeded()))
    }
}

