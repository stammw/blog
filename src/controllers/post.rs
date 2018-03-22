use std::collections::HashMap;
use rocket_contrib::Template;

use db;
use models::Post;
use schema::posts::dsl::*;
use diesel::prelude::*;

#[get("/")]
fn index(db: db::Database) -> Template {
    let mut context = HashMap::new();
    let last_post = posts.limit(5)
        .load::<Post>(&*db)
        .expect("Error loading posts");

    context.insert("posts".to_string(), last_post);

    Template::render("index", &context)
}

#[get("/<post_id>")]
fn get(db: db::Database, post_id: i32) -> Template {
    let post = posts.filter(id.eq(post_id))
        .first::<Post>(&*db)
        .expect("Error loading posts");

    Template::render("post", &post)
}

