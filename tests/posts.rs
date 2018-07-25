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
    let body = "body=Body&title=sometitle";
    post("/post/new", body, true, |res| {
        assert_eq!(res.status(), Status::SeeOther);
        let location = res.headers().get("Location")
            .last().expect("Location is not set"); 
        Regex::new(r"^/post/(\d+)$").unwrap()
            .captures(location).unwrap()
            .get(1).expect("location format invalid").as_str()
            .parse::<i32>().expect("post_id is not a number");
    });
}

#[test]
fn create_post_with_empty_title_fails() {
    let body = "body=Body&title=";
    post("/post/new", body, true, |res| {
        assert_eq!(res.status(), Status::raw(400));
    });
}

// #[test]
// fn gets_one_post() {
//     let mut post_id = -1;

//     dispatch_user_post!("/post/new", format!("body={}&title={}", "Body", "Title"),
//         |_, response: LocalResponse| {
//             assert_eq!(response.status(), Status::SeeOther);
//             let excepted = Regex::new(r"^/post/(\d+)$").unwrap();
//             let location = response.headers().get("Location")
//                 .last().unwrap(); 
//             post_id = excepted.captures(location).unwrap()
//                        .get(1).expect("post id not found in 'Location'")
//                        .as_str().parse::<i32>().unwrap();
//         }
//     );
//     dispatch!(Get, format!("/post/{}", post_id),
//         |_, response: LocalResponse| {
//             assert_eq!(response.status(), Status::Ok);
//         }
//     );
// }
