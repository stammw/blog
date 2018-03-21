#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;

use std::collections::HashMap;
use rocket_contrib::Template;

#[get("/")]
fn index() -> Template {
    let mut context: HashMap<String, String> = HashMap::new();
    context.insert("name".to_string(), "Rocket".to_string());
    Template::render("index", context)
}

fn main() {
    rocket::ignite()
        .attach(Template::fairing())
        .mount("/", routes![index])
        .launch();
}
