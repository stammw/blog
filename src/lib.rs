#![feature(plugin, custom_derive)]
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
extern crate pulldown_cmark;
extern crate time;

pub mod db;
pub mod schema;
pub mod models;
pub mod controllers;

use std::path::{Path, PathBuf};
use rocket_contrib::Template;
use rocket::response::NamedFile;

use controllers::post;
use controllers::login;

#[get("/<file..>")]
fn static_file(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("public/").join(file))
        .ok() 
}

pub fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .attach(Template::fairing())
        .manage(db::init_pool())
        .mount("/", routes![post::index, login::get_form, login::auth_check])
        .mount("/public/", routes![static_file])
        .mount("/post/", routes![post::get, post::new, post::edit_new])
}
