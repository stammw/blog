extern crate stammw_blog;
extern crate rocket;
extern crate rocket_contrib;

use rocket::local::Client;
use rocket::http::{Status, ContentType};

use stammw_blog::auth::UserToken;

fn check_login(body: &str, location: &str, success: bool) {
    let rocket = stammw_blog::rocket();
    let client = Client::new(rocket).unwrap();

    let response = client.post("/login")
        .header(ContentType::Form)
        .body(body)
        .dispatch();

    assert_eq!(response.status(), Status::SeeOther);
    assert_eq!(response.headers().get_one("Location"), Some(location));
    let cookie = response.headers().get_one("Set-Cookie");
    if success {
        assert!(cookie.is_some());
        assert_eq!(&cookie.unwrap()[..5], "user=");
    } else {
        assert!(cookie.is_none());
    }
}

#[test]
fn login_success() {
    check_login("email=user1@email.test&password=password1", "/", true);
}

#[test]
fn login_fails_wrong_password() {
    check_login("email=user1@email.test&password=wrong_password", "/login", false);
}

#[test]
fn login_fails_empty_password() {
    check_login("email=user1@email.test&password=", "/login", false);
}

#[test]
fn login_fails_empty_email() {
    check_login("email=&password=password1", "/login", false);
}

#[test]
fn create_fails_when_username_already_exists() {
    let secret = String::from("test_secret");
    let rocket = stammw_blog::rocket();

    let client = Client::new(rocket).unwrap();
    let mut res = client.post("/user/create")
        .header(ContentType::Form)
        .cookie(UserToken { id: 1, name: "user1".to_string() }.to_cookie(&secret))
        .body("name=testuser&email=exists@domain.com&password=password")
        .dispatch();
    assert_eq!(res.status(), Status::raw(400));
    assert!(res.body_string().unwrap().contains("already exists"));
}

// #[test]
// fn new_users_errors_are_displayed() {
//     let client = client(user_repo_factory);
//     let mut response = client.post("/user/create")
//         .header(ContentType::Form)
//         .private_cookie(UserCookie::create(1, "test_user"))
//         .body("name=&email=@domain.com&password=")
//         .dispatch();
//     assert_eq!(response.status(), Status::raw(400));
//     let content = response.body_string().unwrap();
//     assert!(content.contains("Name shall not be empty"));
//     assert!(content.contains("Email is not valid"));
//     assert!(content.contains("Password should be at least 8 chars"));
// }

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
