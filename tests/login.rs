extern crate stammw_blog;
extern crate rocket;
extern crate rocket_contrib;

use rocket::local::Client;
use rocket::http::{Status, ContentType};

mod helpers;
use helpers::{get, post};

fn check_login(body: &str, location: &str, success: bool) {
    let rocket = stammw_blog::rocket();
    let client = Client::new(rocket).unwrap();

    let response = client.post("/login")
        .header(ContentType::Form)
        .body(body)
        .dispatch();

    let cookie = response.headers().get_one("Set-Cookie");
    if success {
        assert_eq!(response.status(), Status::SeeOther);
        assert_eq!(response.headers().get_one("Location"), Some(location));
        assert!(cookie.is_some());
        assert_eq!(&cookie.unwrap()[..5], "user=");
    } else {
        assert_eq!(response.status(), Status::Unauthorized);
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
fn user_email_stays_on_login_failure() {
    let body = "email=i-should-stay-in-input&password=wrong";
    post("/login", body, false, |res| {
        assert_eq!(res.status(), Status::Unauthorized);
        assert!(res.body_string().unwrap().contains("i-should-stay-in-input"));
    });
}

#[test]
fn logout() {
    get("/logout", true, |res| {
        assert_eq!(res.status(), Status::SeeOther);
        let cookie_header = res.headers().get_one("Set-Cookie");
        assert!(cookie_header.unwrap().contains("user=; Max-Age=0;"));
    })
}

#[test]
fn logout_not_logged() {
    get("/logout", false, |res| {
        assert_eq!(res.status(), Status::SeeOther);
        assert_eq!(res.headers().get_one("Location"), Some("/"));
    })
}
