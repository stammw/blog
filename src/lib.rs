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
pub mod controllers;

use rocket_contrib::Template;

use controllers::post;

pub fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .attach(Template::fairing())
        .manage(db::init_pool())
        .mount("/", routes![post::index])
        .mount("/post/", routes![post::get])
}
