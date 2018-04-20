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
use stammw_blog::controllers::login::UserCookie;

#[macro_use]
mod helpers;

#[test]
fn index_renders() {
    dispatch!(Get, "/", |_, response: LocalResponse| {
        assert_eq!(response.status(), Status::Ok);
    });
}

#[test]
fn create_post() {
    let test_response = |_, response: LocalResponse| {
        assert_eq!(response.status(), Status::SeeOther);
        let excepted_url = Regex::new(r"^/post/\d+$").unwrap();
        let location = response.headers().get("Location").last(); 
        assert!(location.is_some());
        assert!(excepted_url.is_match(location.unwrap()));
    };

    dispatch_user_post!("/post/new",
        format!("body={}&title={}", "Body", "Title"),
        test_response
    );
}

#[test]
fn create_post_with_empty_title_fails() {
    dispatch_user_post!("/post/new",
        format!("body={}&title={}", "Body", ""),
        |_, response: LocalResponse| {
            assert_eq!(response.status(), Status::Ok);
        }
    );
}

#[test]
fn gets_one_post() {
    let mut post_id = -1;
    dispatch_user_post!("/post/new", format!("body={}&title={}", "Body", "Title"),
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
