use std::collections::HashMap;
use rocket::request::Form;
use rocket::response::Redirect;
use rocket_contrib::Template;
use serde::ser::Serialize;
use serde_json;

use db;
use login::UserCookie;
use models::{NewPost, Post};
use schema::posts::dsl::*;
use diesel::prelude::*;
use diesel::insert_into;

fn empty_context() -> HashMap<String, String> {
    HashMap::new()
}

fn user_context<T>(user_data: T) -> HashMap<String, serde_json::Value>
where T: Serialize
{
    let mut context = HashMap::new();
    context.insert("user".to_string(), json!(&user_data));
    context
}

#[get("/")]
fn index(db: db::Database, user_cookie: Option<UserCookie>) -> Template {
    let mut context = user_cookie.map_or(HashMap::new(), |c| user_context(c));
    let last_post: Vec<Post> = posts.limit(50)
        .load::<Post>(&*db)
        .expect("Error loading posts")
        .into_iter()
        .map(|p| p.format())
        .collect();

    context.insert("posts".to_string(), json!(last_post));

    Template::render("index", &context)
}

#[get("/<post_id>")]
fn get(db: db::Database, post_id: i32) -> Template {
    let post = posts.filter(id.eq(post_id))
        .first::<Post>(&*db)
        .expect("Error loading posts"); // TODO return 404

    Template::render("post", &post.format())
}

#[get("/new")]
fn edit_new(_user_cookie: UserCookie) -> Template {
    let context: HashMap<&str, &str> = HashMap::new();
    Template::render("edit_post", &empty_context())
}

#[post("/new", data = "<post_form>")]
fn new(db: db::Database, post_form: Option<Form<NewPost>>) -> Result<Redirect, Template> {
    let post = post_form.unwrap().into_inner();
    let new_post = insert_into(posts)
        .values(&post)
        .get_result::<Post>(&*db)
        .expect("Failed to insert post");

    match post.validate() {
        Ok(_)    => Ok(Redirect::to(format!("/post/{}", new_post.id).as_str())),
        Err(_) => Err(Template::render("edit_post", &new_post)),
    }
}
