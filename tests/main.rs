#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate serde_json;
extern crate stammw_blog;
extern crate rocket;
extern crate rocket_contrib;

use rocket::local::{Client, LocalResponse};
use rocket::http::Method::*;
use rocket::http::Status;

macro_rules! dispatch {
    ($method:expr, $path:expr, $test_fn:expr) => ({
        let client = Client::new(stammw_blog::rocket()).unwrap();
        $test_fn(&client, client.req($method, $path).dispatch());
    })
}

#[test]
fn index_renders() {
    dispatch!(Get, "/", |_, response: LocalResponse| {
        assert_eq!(response.status(), Status::Ok);
    });
}
