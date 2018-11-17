use chrono::Utc;
use rocket::post;
use rocket::request::{Form};
use rocket::response::Redirect;
use rocket::response::status::BadRequest;
use rocket_contrib::json::JsonValue;
use std::collections::HashMap;

use controllers::post;
use auth::UserToken;
use models::NewComment;
use repositories::comments::CommentRepo;
use repositories::posts::PostRepo;

#[derive(FromForm)]
pub struct CommentForm {
    pub user_id: i32,
    pub body: String,
}

impl CommentForm {
    pub fn validate(&self) -> Result<(), HashMap<&'static str, &'static str>> {
        let mut errors = HashMap::new();
        if self.body.is_empty() {
            errors.insert("body", "Body shall not be emty");
        }
        if !errors.is_empty() {
            Err(errors)
        } else {
            Ok(())
        }
    }
}

#[post("/<post_id>/comment", data = "<form>")]
pub fn new(
    post_id: i32,
    form: Form<CommentForm>,
    comment_repo: CommentRepo,
    post_repo: PostRepo,
    user: UserToken,
) -> Result<Redirect, BadRequest<JsonValue>> {
    let comment = form.into_inner();

    if let Err(e) = comment.validate() {
        return Err(BadRequest(Some(json!(e))));
    }

    let post = match post_repo.get(post_id) {
        Ok(post) => post,
        Err(_) => return Err(BadRequest(None)),
    };

    if comment.user_id != user.id {
        return Err(BadRequest(None));
    }

    let comment = NewComment {
        user_id: user.id,
        post_id: post_id,
        body: comment.body,
        creation_date: Utc::now().naive_utc(),
    };

    comment_repo.insert(&comment);
    Ok(Redirect::to(uri!(post::get_by_slug: post.slug)))
}
