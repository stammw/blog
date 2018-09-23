extern crate stammw_blog;
extern crate rocket;
extern crate rocket_contrib;

use rocket::http::Status;

mod helpers;
use helpers::post;

#[test]
fn can_comment_on_post() {
    let body = "body=very_intedresting";
    post("/post/1/comment", body, true, |res| {
        assert_eq!(res.status(), Status::Created);
    });
}

#[test]
fn cannot_comment_when_not_logged() {
    let body = "body=very_intedresting";
    post("/post/1/comment", body, false, |res| {
        assert_eq!(res.status(), Status::raw(401));
    });
}

#[test]
fn cannot_comment_on_non_existent_post() {
    let body = "body=very_intedresting";
    post("/post/999/comment", body, true, |res| {
        assert_eq!(res.status(), Status::BadRequest);
    });
}
