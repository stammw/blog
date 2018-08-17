extern crate stammw_blog;
extern crate rocket;
extern crate rocket_contrib;
extern crate regex;

use rocket::http::Status;

mod helpers;
use helpers::get;

#[test]
fn index_renders() {
    get("/", false, |res| assert_eq!(res.status(), Status::Ok));
}

#[test]
fn assets_load() {
    get("/public/app.js", false, |res| assert_eq!(res.status(), Status::Ok));
}

#[test]
fn login_indicator_displays() {
    get("/", true, |res| {
        assert_eq!(res.status(), Status::Ok);
        let body = res.body_string().unwrap();
        assert!(body.contains("<li>user1</li>"));
    });
}
