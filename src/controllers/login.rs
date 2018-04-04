use std::collections::HashMap;
use time::Duration;

use rocket::Outcome;
use rocket::http::{Cookie, Cookies, Status};
use rocket::request::{Form, FromRequest, Request};
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

pub struct UserCookie(u32);

impl<'a, 'r> FromRequest<'a, 'r> for UserCookie {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> Outcome<UserCookie, (Status, ()), ()> {
        if request.cookies().get_private("user_id").is_some() {
            println!("Cookie is some");
        } else {
            println!("Cookie is none");
        }

        let cookie = request.cookies()
            .get_private("user_id")
            .and_then(|cookie| cookie.value().parse().ok())
            .map(|id| UserCookie(id));
        match cookie {
            Some(c) => Outcome::Success(c),
            None    => Outcome::Failure((Status::BadRequest, ())),
        }
    }
}


#[post("/login", data = "<login_form>")]
fn auth_check(mut cookies: Cookies, login_form: Option<Form<Login>>) -> Redirect {
    let login = login_form.unwrap().into_inner();
    if login.email == "yep@yep.yep" && login.password == "yep" {
        let cookie = Cookie::build("user_id", "1")
            .max_age(Duration::minutes(1))
            // .secure(true) // TODO uncomment once TLS is on
            .finish();
        cookies.add_private(cookie);
        Redirect::to("/")
    } else {
        Redirect::to("/login")
    }
}
