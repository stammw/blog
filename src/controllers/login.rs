use std::collections::HashMap;
use time::Duration;

use rocket::Outcome;
use rocket::http::{Cookie, Cookies, Status};
use rocket::request::{Form, FromRequest, Request};
use rocket::response::Redirect;
use rocket_contrib::Template;
use serde_json::{self, Value};
use serde_json::map::Map;

#[derive(Serialize, Deserialize)]
pub struct UserCookie {
    id: u32,
    name: String,
}

impl UserCookie {
    pub fn context(cookie: &Self) -> HashMap<String, Value> {
        let mut context = HashMap::new();
        context.insert("user".to_string(), json!(cookie));
        context
    }

    pub fn context_or(cookie: &Option<Self>) -> HashMap<String, Value> {
        match cookie {
            Some(c) => UserCookie::context(&c),
            None    => HashMap::new(),
        }
    }

    pub fn wrap(self, context: Value) -> Value {
        match context {
            Value::Object(mut obj) => {
                obj.insert("user".to_string(), json!(self));
                Value::Object(obj)
            },
            _ => {
                let mut object_ctx: Map<String, Value> = Map::new();
                object_ctx.insert("data".to_string(), context);
                Value::Object(object_ctx)
            }
        }

    }

    pub fn from_cookies(cookies: &mut Cookies) -> Option<UserCookie> {
        cookies.get_private("user_id")
            .and_then(|cookie| cookie.value().parse().ok())
            .map(|json| serde_json::from_value(json).unwrap())
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for UserCookie {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> Outcome<UserCookie, (Status, ()), ()> {
        let cookie = UserCookie::from_cookies(&mut request.cookies());

        match cookie {
            Some(c) => Outcome::Success(c),
            None    => Outcome::Failure((Status::BadRequest, ())),
        }
    }
}

#[derive(FromForm)]
struct Login {
    email: String,
    password: String,
}

#[get("/login")]
fn form_already_logged(_user_cookie: UserCookie) -> Redirect {
        Redirect::to("/")
}

#[get("/login", rank = 2)]
fn form() -> Template {
    let context: HashMap<String, String> = HashMap::new();
    Template::render("login", &context)
}

#[post("/login")]
fn auth_already_logged(_user_cookie: UserCookie) -> Redirect {
        Redirect::to("/")
}

#[post("/login", data = "<login_form>", rank = 2)]
fn auth(mut cookies: Cookies, login_form: Option<Form<Login>>) -> Redirect {
    let login = login_form.unwrap().into_inner();

    if login.email == "yep@yep.yep" && login.password == "yep" {
        let cookie = Cookie::build("user_id", json!({
                    "id": 1,
                    "name": "stammw"
                  }).to_string())
            .max_age(Duration::days(1))
            // .secure(true) // TODO uncomment once TLS is on
            .finish();
        cookies.add_private(cookie);
        Redirect::to("/")
    } else {
        Redirect::to("/login")
    }
}
