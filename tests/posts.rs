#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate diesel;
#[macro_use]
extern crate serde_json;
extern crate stammw_blog;
extern crate rocket;
extern crate rocket_contrib;
extern crate regex;

use diesel::prelude::*;
use rocket::local::{Client, LocalRequest, LocalResponse};
use rocket::http::Method::*;
use rocket::http::{Status, ContentType};
use rocket::request::FromRequest;
use rocket::response::Redirect;
use regex::Regex;
use stammw_blog::db::Database;
use stammw_blog::models::{Post, NewPost, User, NewUser};
use stammw_blog::controllers::login::UserCookie;
use stammw_blog::schema::posts::dsl::posts;
use stammw_blog::controllers;
use stammw_blog::repositories::post::PostRepository;
use stammw_blog::repositories::user::UserRepository;

#[macro_use]
mod helpers;

struct PostRepositoryMock;

impl PostRepository for PostRepositoryMock {
    fn all(&self, _limit: i64) -> Vec<Post> { Vec::new() }
    fn get(&self, _post_id: i32) -> Option<Post> { None }
    fn insert(&self, _post: &NewPost) -> Post { unimplemented!(); }
}

struct UserRepositoryMock;

impl UserRepository for UserRepositoryMock {
    fn all(&self) -> Vec<User> { unimplemented!(); }
    fn get(&self, user_id: i32) -> Option<User> { unimplemented!(); }
    fn insert(&self, user: &NewUser) -> User { unimplemented!(); }
    fn count(&self) -> i64 { 0 }
}

#[test]
fn index_renders() {
    let create_post = |request: &LocalRequest| {
        let db = Database::from_request(&request.inner()).unwrap();
        let post = NewPost {
            title: "Test Post".to_string(),
            body: "# Test post body\nempty".to_string(),
            published: true,
        };
        diesel::insert_into(posts).values(&post).execute(&*db).unwrap();
    };

    dispatch_request!(Get, "/", create_post,  |_, response: LocalResponse| {
        assert_eq!(response.status(), Status::Ok);
    });
}

#[test]
fn get_not_found_when_no_post() {
    let mocked_repo = Box::new(PostRepositoryMock);
    let response = controllers::post::get(mocked_repo, 0, None);
    assert!(response.is_err());
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
