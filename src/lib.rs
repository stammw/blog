#![feature(plugin, custom_derive, decl_macro)]
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
extern crate time;

pub mod controllers;
pub mod db;
pub mod models;
pub mod repositories;
pub mod schema;

use rocket::response::NamedFile;
use rocket_contrib::Template;
use std::path::{Path, PathBuf};

use controllers::login;
use controllers::post;

#[get("/<file..>")]
fn static_file(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("public/").join(file)).ok()
}

pub fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .attach(Template::fairing())
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
}
