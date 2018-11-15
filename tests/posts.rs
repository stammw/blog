extern crate chrono;
extern crate rocket;
extern crate rocket_contrib;
extern crate regex;
extern crate stammw_blog;
#[macro_use]
extern crate lazy_static;

use chrono::{Duration, Utc};
use regex::Regex;
use rocket::http::Status;
use stammw_blog::repositories::posts::PostRepo;
use stammw_blog::models::Post;

mod helpers;
use helpers::{get, post, post_req};

lazy_static! {
    static ref ID_REGEX: Regex   = Regex::new(r"^/post/(\d+)$").unwrap();
    static ref SLUG_REGEX: Regex = Regex::new(r"^/post/([\w-]+)$").unwrap();
}

fn parse_location_id(location: &str) -> Option<i32> {
    ID_REGEX.captures(location)?.get(1)?.as_str()
        .parse::<i32>().ok()
}

fn parse_location_slug<'a>(location: &'a str) -> Option<&'a str> {
    SLUG_REGEX.captures(location)?
        .get(1).map(|c| c.as_str())
}

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
        let slug = parse_location_slug(location).expect("invalid location");
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
        let id = parse_location_id(location).expect("invalid location");
        assert!(id > 0);
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
        let id = parse_location_id(location).expect("invalid location");
        assert_eq!(id, 45);
    });
}

#[test]
fn update_post_published() {
    let body = "body=Body&title=sometitle%20of%20a%20twice%20updated%20post&published=true";
    post("/post/45", body, true, |res| {
        assert_eq!(res.status(), Status::SeeOther);
        let location = res.headers().get("Location")
            .last().expect("Location is not set");
        let slug = parse_location_slug(location).expect("location format invalid");
        assert_eq!(slug, "sometitle-of-a-twice-updated-post");
    });
}

#[test]
fn list_displays_all_posts() {
    get("/post/list", true, |res| {
        assert_eq!(res.status(), Status::Ok);
        let body = res.body_string().unwrap();
        assert!(body.contains("title1"));
        assert!(body.contains("title2"));
        assert!(body.contains("title3"));
        assert!(body.contains("title45"));
    });
}

#[test]
fn list_needs_auth() {
    get("/post/list", false, |res| {
        assert_eq!(res.status(), Status::Unauthorized);
    });
}

#[test]
fn list_only_unpublished() {
    get("/post/list?published=false", true, |res| {
        assert_eq!(res.status(), Status::Ok);
        let body = res.body_string().unwrap();
        assert!(!body.contains("title1"));
        assert!(!body.contains("title2"));
        assert!(body.contains("title45"));
        assert!(body.contains("title3"));
    });
}

#[test]
fn list_only_published() {
    get("/post/list?published=true", true, |res| {
        assert_eq!(res.status(), Status::Ok);
        let body = res.body_string().unwrap();
        assert!(body.contains("title1"));
        assert!(body.contains("title2"));
        assert!(!body.contains("title45"));
        assert!(!body.contains("title3"));
    });
}

#[test]
fn list_no_params() {
    get("/post/list?", true, |res| {
        assert_eq!(res.status(), Status::Ok);
        let body = res.body_string().unwrap();
        assert!(body.contains("title1"));
        assert!(body.contains("title2"));
        assert!(body.contains("title45"));
        assert!(body.contains("title3"));
    });
}


fn check_post(url: &str, body: &str) -> Post {
    let mut post = None;
    post_req(url, body, true, |req| {
        let repo = req.inner().guard::<PostRepo>().unwrap();

        let res = req.dispatch();
        assert_eq!(res.status(), Status::SeeOther);
        let location = res.headers().get("Location")
            .last().expect("Location is not set");
        println!("location: {}", location);

        if let Some(id) = parse_location_id(location) {
            post = repo.get(id).ok();
        } else if let Some(slug) = parse_location_slug(location) {
            post = repo.get_by_slug(slug)
                .map(|p| p.0).ok();
        }

    });
    post.unwrap()
}

#[test]
fn creation_date_set_on_creation() {
    let body = "body=Body&title=creation_date_set_on_creation&published=false";
    let post = check_post("/post/new", body);
    let created_since = post.creation_date.signed_duration_since(Utc::now().naive_utc());
    assert!(created_since < Duration::seconds(5));
    assert!(post.publication_date.is_none());
    assert!(post.edition_date.is_none());
}

#[test]
fn publication_date_set_on_publishing_creation() {
    let body = "body=Body&title=publication_date_set_on_publishing_creation&published=true";
    let post = check_post("/post/new", body);
    let created_since = post.creation_date.signed_duration_since(Utc::now().naive_utc());
    let published_since = post.publication_date.expect("publication date should be set")
        .signed_duration_since(Utc::now().naive_utc());
    assert!(created_since < Duration::seconds(5));
    assert!(published_since < Duration::seconds(5));
    assert!(post.edition_date.is_none());
}

#[test]
fn edition_date_set_on_edition() {
    let body = "body=Body&title=edition_date_set_on_edition&published=false";
    let new = check_post("/post/new", body);
    let post = check_post(&format!("/post/{}", new.id), body);
    let created_since = post.creation_date.signed_duration_since(Utc::now().naive_utc());
    let edited_since = post.edition_date.expect("edition date should be set")
        .signed_duration_since(Utc::now().naive_utc());
    assert!(created_since < Duration::seconds(5));
    assert!(edited_since < Duration::seconds(5));
    assert!(post.publication_date.is_none());
}

#[test]
fn publication_date_set_on_publishing_edition() {
    let mut body = "body=Body&title=dition_date_set_on_publishing_edition&published=false";
    let new = check_post("/post/new", body);
    body = "body=Body&title=dition_date_set_on_publishing_edition&published=true";
    let post = check_post(&format!("/post/{}", new.id), body);
    let created_since = post.creation_date.signed_duration_since(Utc::now().naive_utc());
    let edited_since = post.edition_date.expect("edition date should be set")
        .signed_duration_since(Utc::now().naive_utc());
    let published_since = post.publication_date.expect("publication date should be set")
        .signed_duration_since(Utc::now().naive_utc());
    assert!(created_since < Duration::seconds(5));
    assert!(edited_since < Duration::seconds(5));
    assert!(published_since < Duration::seconds(5));
}
