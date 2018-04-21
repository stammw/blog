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
use regex::Regex;
use stammw_blog::db::Database;
use stammw_blog::models::{Post, NewPost};
use stammw_blog::controllers::login::UserCookie;
use stammw_blog::schema::users::dsl::users;
use stammw_blog::schema::posts::dsl::posts;

#[macro_use]
mod helpers;

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
fn index_and_no_post_nor_users_redirects_to_create_user() {
    let delete_all = |request: &LocalRequest| {
        let db = Database::from_request(&request.inner()).unwrap();
        diesel::delete(users).execute(&*db).unwrap();
        diesel::delete(posts).execute(&*db).unwrap();

        let last_post: Vec<Post> = posts.limit(50)
            .load::<Post>(&*db)
            .expect("Error loading posts")
            .into_iter()
            .collect();
        println!("All the DB: {}", json!(last_post))
    };

    dispatch_request!(Get, "/", delete_all, |_, response: LocalResponse| {
        assert_eq!(response.status(), Status::SeeOther);
        assert_eq!(
            response.headers().get("Location").last().unwrap(),
            "/create_user"
        ); 
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
