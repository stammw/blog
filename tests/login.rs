extern crate stammw_blog;
extern crate rocket;
extern crate rocket_contrib;

use rocket::local::Client;
use rocket::http::{Status, ContentType};

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
