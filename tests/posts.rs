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
fn index_display_user() {
    get("/", true, |res| {
        assert_eq!(res.status(), Status::Ok);
        assert!(res.body_string().unwrap().contains("<li>user1</li>"));
    });
}

#[test]
fn create_post_success_location_slug_if_published() {
    let body = "body=Body&title=sometitle%20of%20a%20post&published=true";
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
fn create_post_success_location_id_if_unpublished() {
    let body = "body=Body&title=sometitle%20of%20another%20post&published=false";
    post("/post/new", body, true, |res| {
        assert_eq!(res.status(), Status::SeeOther);
        let location = res.headers().get("Location")
            .last().expect("Location is not set"); 
        let id = Regex::new(r"^/post/(\d+)$").unwrap()
            .captures(location).unwrap()
            .get(1).expect("location format invalid").as_str();
        assert!(id.parse::<u64>().unwrap() > 0);
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
fn gets_one_post_by_slug() {
    get("/post/title2-the-second-post", false, |res| {
        assert_eq!(res.status(), Status::Ok);
    })
}

#[test]
fn get_one_post_redenred() {
    get("/post/title1", true, |res| {
        assert_eq!(res.status(), Status::Ok);
        let body = res.body_string().unwrap();
        assert!(body.contains("<h1>body1</h1>"));
    })
}

#[test]
fn gets_one_post_by_slug_fails_when_not_published() {
    get("/post/3", false, |res| {assert_eq!(res.status(), Status::NotFound)})
}

#[test]
fn gets_one_post_by_id_success_when_not_published_and_logged() {
    get("/post/3", true, |res| {assert_eq!(res.status(), Status::Ok)})
}

#[test]
fn gets_one_post_by_id_fails_when_not_published_and_not_logged() {
    get("/post/3", false, |res| {assert_eq!(res.status(), Status::NotFound)})
}

#[test]
fn edit_post_needs_auth() {
    get("/post/3/edit", false, |res| {assert_eq!(res.status(), Status::Unauthorized)})
}

#[test]
fn edit_post() {
    get("/post/3/edit", true, |res| {
        assert_eq!(res.status(), Status::Ok);
        let body = res.body_string().unwrap();
        assert!(body.contains("title3"));
        assert!(body.contains("# body3"));
    })
}

#[test]
fn update_post_unpublished() {
    let body = "body=Body&title=sometitle%20of%20an%20updated%20post&published=false";
    post("/post/45", body, true, |res| {
        assert_eq!(res.status(), Status::SeeOther);
        let location = res.headers().get("Location")
            .last().expect("Location is not set"); 
        let id = Regex::new(r"^/post/(\d+)$").unwrap()
            .captures(location).unwrap()
            .get(1).expect("location format invalid").as_str();
        assert_eq!(id, "45");
        // assert_eq!(slug, "sometitle-of-an-updated-post");
    });
}

#[test]
fn cupdate_post_published() {
    let body = "body=Body&title=sometitle%20of%20a%20twice%20updated%20post&published=true";
    post("/post/45", body, true, |res| {
        assert_eq!(res.status(), Status::SeeOther);
        let location = res.headers().get("Location")
            .last().expect("Location is not set"); 
        let slug = Regex::new(r"^/post/([\w-]+)$").unwrap()
            .captures(location).unwrap()
            .get(1).expect("location format invalid").as_str();
        assert_eq!(slug, "sometitle-of-a-twice-updated-post");
    });
}
