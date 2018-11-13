use rocket::http::{Cookie, Cookies};
use rocket::http::Status;
use rocket::request::{Form};
use rocket::response::Redirect;
use rocket::response::status;
use rocket::{get, post, State};
use rocket_contrib::templates::Template;
use repositories::users::UserRepo;
use auth::{ ForwardUserToken, UserToken };
use argon2rs::argon2i_simple;

use Secret;

use base64;

#[derive(FromForm)]
pub struct Login {
    email: String,
    password: String,
}

#[get("/login", rank = 1)]
pub fn form_already_logged(_user: ForwardUserToken) -> Redirect {
    Redirect::to("/")
}

#[get("/login", rank = 2)]
pub fn form() -> Template {
    Template::render("login", ())
}

#[post("/login", rank = 1)]
pub fn auth_already_logged(_user: ForwardUserToken) -> Redirect {
    Redirect::to("/")
}

#[post("/login", data = "<login_form>", rank = 2)]
pub fn auth(mut cookies: Cookies, login_form: Option<Form<Login>>, repo: UserRepo, secret: State<Secret>)
        -> Result<Redirect, status::Custom<Template>>{
    let login = login_form.unwrap().into_inner();

    let failure = || {
        Err(status::Custom(
            Status::Unauthorized,
            Template::render("login", json!({ "email": login.email })))
        )
    };

    match repo.get_by_email(&login.email) {
        Some(ref user) => {
            let password = argon2i_simple(&login.password, &secret.0);
            let hashed = base64::encode(&password);
            if hashed == user.password {
                let jwt = UserToken { id: user.id, name: user.name.to_owned() }.to_jwt(&secret.0);
                cookies.add(Cookie::new("user", jwt));
                Ok(Redirect::to("/"))
            } else {
                failure()
            }
        },
        _ => failure()
    }
}

#[get("/logout")]
pub fn logout(user: Option<UserToken>, mut cookies: Cookies, secret: State<Secret>) -> Redirect {
    if let Some(user) = user {
        cookies.remove(user.to_cookie(&secret.0));
    }
    Redirect::to("/")
}
