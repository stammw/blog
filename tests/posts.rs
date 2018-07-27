extern crate stammw_blog;
extern crate rocket;
extern crate rocket_contrib;
extern crate regex;

use regex::Regex;
use rocket::http::Status;

mod helpers;
use helpers::{get,post};

#[test]
fn index_renders() {
    get("/", false, |res| assert_eq!(res.status(), Status::Ok));
}

#[test]
fn create_post_success() {
    let body = "body=Body&title=sometitle%20of%20a%20post";
    post("/post/new", body, true, |res| {
        assert_eq!(res.status(), Status::SeeOther);
        let location = res.headers().get("Location")
            .last().expect("Location is not set"); 
        let slug = Regex::new(r"^/post/([\w-]+)$").unwrap()
            .captures(location).unwrap()
            .get(1).expect("location format invalid").as_str();
        assert_eq!(slug, "sometitle-of-a-post");
    });
}

#[test]
fn create_post_with_empty_title_fails() {
    let body = "body=Body&title=";
    post("/post/new", body, true, |res| {
        assert_eq!(res.status(), Status::raw(400));
    });
}

#[test]
fn gets_one_post_by_id() {
    get("/post/3", false, |res| {
        assert_eq!(res.status(), Status::Ok);
        let body = res.body_string().unwrap();
        assert!(body.contains("<h1>body3</h1>"));
    })
}

#[test]
fn gets_one_post_by_slug() {
    get("/post/title2-the-second-post", false, |res| {
        assert_eq!(res.status(), Status::Ok);
    })
}
