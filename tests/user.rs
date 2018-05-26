extern crate stammw_blog;
extern crate rocket;
extern crate rocket_contrib;

use stammw_blog::controllers::user;
use stammw_blog::controllers::login::UserCookie;
use stammw_blog::models::{NewUser, User};
use stammw_blog::repositories::users::UserRepository;

struct UserRepositoryMock {
    users: Vec<User>,
}

impl UserRepository for UserRepositoryMock {
    fn all(&self, _limit: i64) -> Vec<User> { unimplemented!(); }
    fn get(&self, _user_id: i32) -> Option<User> { unimplemented!(); }
    fn insert(&self, _user: &NewUser) -> User { unimplemented!(); }
    fn count(&self) -> i64 { self.users.len() as i64 }
}

fn user_repo() -> Box<UserRepository> {
    Box::new(UserRepositoryMock {
        users: vec![
            User {
                id: 12,
                name: String::from("testuser"),
                email: String::from("email"),
                password: String::from("password"),
            }
        ]
    })
}

fn user_repo_empty() -> Box<UserRepository> {
    Box::new(UserRepositoryMock { users: Vec::new() })
}

#[test]
fn new_success_when_user_logged_in() {
    let cookie = Some(UserCookie { id: 1, name: String::from("testuser")});
    let res = user::new(user_repo(), cookie);
    assert!(res.is_ok());
}

#[test]
fn new_fails_when_not_logged_in_and_some_user_exist() {
    let res = user::new(user_repo(), None);
    assert!(res.is_err());
}

#[test]
fn new_succes_when_not_logged_in_and_no_user_exist() {
    let res = user::new(user_repo_empty(), None);
    assert!(res.is_ok());
}
