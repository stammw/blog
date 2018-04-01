#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate serde_json;
extern crate stammw_blog;
extern crate rocket;
extern crate rocket_contrib;
extern crate regex;

use rocket::local::{Client, LocalResponse};
use rocket::http::Method::*;
use rocket::http::{Status, ContentType};
use regex::Regex;

macro_rules! dispatch {
    ($method:expr, $path:expr, $test_fn:expr) => ({
        let client = Client::new(stammw_blog::rocket()).unwrap();
        $test_fn(&client, client.req($method, $path).dispatch());
    })
}

macro_rules! dispatch_post {
    ($path:expr, $data:expr, $test_fn:expr) => ({
        let client = Client::new(stammw_blog::rocket()).unwrap();
        $test_fn(&client, client.post($path)
                 .header(ContentType::Form)
                 .body(&$data)
                 .dispatch());
    })
}

#[test]
fn index_renders() {
    dispatch!(Get, "/", |_, response: LocalResponse| {
        assert_eq!(response.status(), Status::Ok);
    });
}

#[test]
fn create_post() {
    dispatch_post!("/post/new",
        format!("body={}&title={}", "Body", "Title"),
        |_, response: LocalResponse| {
            assert_eq!(response.status(), Status::SeeOther);
            let excepted_url = Regex::new(r"^/post/\d+$").unwrap();
            let location = response.headers().get("Location").last(); 
            assert!(location.is_some());
            assert!(excepted_url.is_match(location.unwrap()));
        }
    );
}

#[test]
fn create_post_with_empty_title_fails() {
    dispatch_post!("/post/new",
        format!("body={}&title={}", "Body", ""),
        |_, response: LocalResponse| {
            assert_eq!(response.status(), Status::Ok);
        }
    );
}

#[test]
fn gets_one_post() {
    let mut post_id = -1;
    dispatch_post!("/post/new", format!("body={}&title={}", "Body", "Title"),
        |_, response: LocalResponse| {
            assert_eq!(response.status(), Status::SeeOther);
            let excepted = Regex::new(r"^/post/(\d+)$").unwrap();
            let location = response.headers().get("Location")
                .last().unwrap(); 
            post_id = excepted.captures(location).unwrap()
                       .get(1).expect("post id not found in 'Location'")
                       .as_str().parse::<i32>().unwrap();
        }
    );
    dispatch!(Get, format!("/post/{}", post_id),
        |_, response: LocalResponse| {
            assert_eq!(response.status(), Status::Ok);
        }
    );
}
