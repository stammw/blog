#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate serde_json;
extern crate stammw_blog;
extern crate rocket;
extern crate rocket_contrib;

use rocket::local::{Client, LocalResponse};
use rocket::http::Method::*;
use rocket::http::{Status, ContentType};

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
#[ignore]
fn gets_one_post() {
    dispatch!(Get, "/post/1", |_, response: LocalResponse| {
        assert_eq!(response.status(), Status::Ok);
    });
}
