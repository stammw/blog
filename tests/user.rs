extern crate stammw_blog;
extern crate rocket;
extern crate rocket_contrib;

use stammw_blog::controllers::user;
use stammw_blog::controllers::login::UserCookie;
use stammw_blog::models::{NewUser, User};
use stammw_blog::repositories::users::{
    UserRepositoryTrait, UserRepository, UserRepositoryFactory
};
use rocket::local::{Client, LocalResponse};
use rocket::http::{Status, ContentType};

#[macro_use]
mod helpers;

struct UserRepositoryMock {
    users: Vec<User>,
}

impl UserRepositoryTrait for UserRepositoryMock {
    fn all(&self, _limit: i64) -> Vec<User> { unimplemented!(); }
    fn get(&self, _user_id: i32) -> Option<User> { unimplemented!(); }
    fn get_by_name(&self, name: &String) -> Option<User> {
        self.users.iter().find(|u| u.name == *name).map(|u| u.clone())
    }
    fn insert(&self, _user: &NewUser) -> User { unimplemented!(); }
    fn count(&self) -> i64 { self.users.len() as i64 }
}

fn user_repo_factory(db: Option<stammw_blog::db::Database>) -> UserRepository {
    Box::new(UserRepositoryMock {
        users: vec![
            User {
                id: 12,
                name: String::from("testuser"),
                email: String::from("email"),
                password: String::from("password"),
            }
        ]
    }) as UserRepository
}

fn client(repo_factory: UserRepositoryFactory) -> Client {
    let rocket = stammw_blog::rocket_stateless()
        .manage(repo_factory);
    Client::new(rocket).unwrap()
}

fn user_repo_empty() -> UserRepository {
    Box::new(UserRepositoryMock { users: Vec::new() })
}

#[test]
fn create_fails_when_username_already_exists() {
    let client = client(user_repo_factory);
    let mut res = client.post("/user/create")
             .header(ContentType::Form)
             .private_cookie(UserCookie::create(1, "test_user"))
             .body("name=testuser&email=exists@domain.com&password=password")
             .dispatch();
    assert_eq!(res.status(), Status::raw(400));
    assert!(res.body_string().unwrap().contains("already exists"));
}

#[test]
fn new_users_errors_are_displayed() {
    dispatch_user_post!("/user/create", "name=&email=&password=", |_, mut response: LocalResponse| {
        assert_eq!(response.status(), Status::raw(400));
        let mut content = String::new();
        response.body().unwrap().into_inner()
            .read_to_string(&mut content);
        assert!(content.contains("Name shall not be empty"));
    }) 
}

// #[test]
// fn new_success_when_user_logged_in() {
//     let cookie = Some(UserCookie { id: 1, name: String::from("testuser")});
//     let res = user::new(user_repo(), cookie);
//     assert!(res.is_ok());
// }

// #[test]
// fn new_fails_when_not_logged_in_and_some_user_exist() {
//     let res = user::new(user_repo(), None);
//     assert!(res.is_err());
// }

// #[test]
// fn new_succes_when_not_logged_in_and_no_user_exist() {
//     let res = user::new(user_repo_empty(), None);
//     assert!(res.is_ok());
// }

// #[test]
// fn create_success_when_user_logged_in() {
//     let cookie = Some(UserCookie { id: 1, name: String::from("testuser")});
//     let res = user::create(new_user_form(), user_repo(), cookie);
//     assert!(res.is_ok());
// }

// #[test]
// fn create_fails_when_not_logged_in_and_some_user_exist() {
//     let res = user::create(None, user_repo(), None);
//     assert!(res.is_err());
// }

// #[test]
// fn create_succes_when_not_logged_in_and_no_user_exist() {
//     let res = user::create(None, user_repo_empty(), None);
//     assert!(res.is_ok());
// }
