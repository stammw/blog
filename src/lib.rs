#![feature(plugin, proc_macro_hygiene, custom_derive, decl_macro)]
#![allow(proc_macro_derive_resolution_fallback)] // This can be removed after diesel-1.4

#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate diesel;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate log;

extern crate dotenv;
extern crate chrono;
extern crate pulldown_cmark;
extern crate r2d2;
extern crate r2d2_diesel;
#[macro_use]
extern crate rocket;
extern crate rocket_contrib;
extern crate serde;
extern crate base64;
extern crate time;
extern crate regex;
extern crate frank_jwt;
extern crate argon2rs;
extern crate slug;

pub mod controllers;
pub mod db;
pub mod models;
pub mod repositories;
pub mod schema;
pub mod auth;
pub mod pagination;

use rocket::{get, routes};
use rocket::response::NamedFile;
use rocket::fairing::AdHoc;
use rocket_contrib::templates::Template;
use std::path::{Path, PathBuf};

use controllers::login;
use controllers::post;
use controllers::user;
use controllers::comment;

#[get("/<file..>")]
fn static_file(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("public/").join(file)).ok()
}

pub struct Secret(String);

pub fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .attach(Template::fairing())
        .manage(repositories::users::factory)
        .manage(repositories::posts::factory)
        .manage(db::init_pool())
        .mount(
            "/",
            routes![
                post::index,
                post::index_page,
                login::form_already_logged,
                login::form,
                login::auth_already_logged,
                login::auth,
                login::logout,
            ],
        )
        .mount("/public", routes![static_file])
        .mount("/post", routes![
            post::get,
            post::get_by_slug,
            post::new,
            post::edit_new,
            post::edit,
            post::update,
            post::list,
            post::list_with_params,
            comment::new,
        ])
        .mount("/user", routes![user::new, user::create])
        .attach(AdHoc::on_attach("Cookies manager", |rocket| {
            println!("Adding token managed state from config...");
            let token_val = rocket.config().get_str("secret")
                .expect("Missing secret configuration").to_string();
            Ok(rocket.manage(Secret(token_val)))
        }))
}
