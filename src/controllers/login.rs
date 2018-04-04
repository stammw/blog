use std::collections::HashMap;
use rocket::request::Form;
use rocket::response::Redirect;
use rocket_contrib::Template;

#[get("/login")]
fn get_form() -> Template {
    let context: HashMap<String, String> = HashMap::new();
    Template::render("login", &context)
}

#[derive(FromForm)]
struct Login {
    email: String,
    password: String,
}

#[post("/login", data = "<login_form>")]
fn auth_check(login_form: Option<Form<Login>>) -> Redirect {
    let login = login_form.unwrap().into_inner();
    if login.email == "yep@yep.yep" && login.password == "yep" {
        Redirect::to("/")
    } else {
        Redirect::to("/login")
    }
}
