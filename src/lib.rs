#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate serde;
extern crate serde_json;
#[macro_use] extern crate serde_derive;

extern crate rocket;
extern crate rocket_contrib;
extern crate r2d2_diesel;
#[macro_use] extern crate diesel;
extern crate r2d2;
extern crate dotenv;

pub mod db;
pub mod schema;
pub mod models;

use std::collections::HashMap;
use rocket_contrib::Template;

use self::models::Post;
use self::schema::posts::dsl::*;
use self::diesel::prelude::*;

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

pub fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .attach(Template::fairing())
        .manage(db::init_pool())
        .mount("/", routes![index])
        .mount("/post/", routes![get])
}

