#![feature(plugin, custom_derive, decl_macro, extern_prelude)]
#![plugin(rocket_codegen)]

#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate diesel;

extern crate dotenv;
extern crate pulldown_cmark;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate rocket;
extern crate rocket_contrib;
extern crate serde;
extern crate base64;
extern crate time;
extern crate regex;
extern crate frank_jwt;
extern crate argon2rs;

pub mod controllers;
pub mod db;
pub mod models;
pub mod repositories;
pub mod schema;
mod auth;

use rocket::response::NamedFile;
use rocket::fairing::AdHoc;
use rocket_contrib::Template;
use std::path::{Path, PathBuf};

use controllers::login;
use controllers::post;
use controllers::user;
use repositories::users;

#[get("/<file..>")]
fn static_file(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("public/").join(file)).ok()
}

struct Secret(String);

pub fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .attach(Template::fairing())
        .manage(users::factory)
        .manage(db::init_pool())
        .mount(
            "/",
            routes![
                post::index,
                login::form_already_logged,
                login::form,
                login::auth_already_logged,
                login::auth,
            ],
        )
        .mount("/public/", routes![static_file])
        .mount("/post/", routes![post::get, post::new, post::edit_new])
        .mount("/user/", routes![user::new, user::create])
        .attach(AdHoc::on_attach(|rocket| {
            println!("Adding token managed state from config...");
            let token_val = rocket.config().get_str("secret")
                .expect("Missing secret configuration").to_string();
            Ok(rocket.manage(Secret(token_val)))
        }))
}
