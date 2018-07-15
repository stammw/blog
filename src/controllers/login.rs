use rocket::http::{Cookie, Cookies};
use rocket::request::Form;
use rocket::response::Redirect;
use rocket::State;
use rocket_contrib::Template;
use repositories::users::UserRepo;
use auth::{ ForwardUserToken, UserToken };
use argon2rs::argon2i_simple;
use Secret;

use base64;

#[derive(FromForm)]
struct Login {
    email: String,
    password: String,
}

#[get("/login", rank = 1)]
fn form_already_logged(_user: ForwardUserToken) -> Redirect {
    Redirect::to("/")
}

#[get("/login", rank = 2)]
fn form() -> Template {
    Template::render("login", ())
}

#[post("/login", rank = 1)]
fn auth_already_logged(_user: ForwardUserToken) -> Redirect {
    Redirect::to("/")
}

#[post("/login", data = "<login_form>", rank = 2)]
fn auth(mut cookies: Cookies, login_form: Option<Form<Login>>, repo: UserRepo, secret: State<Secret>) -> Redirect {
    let login = login_form.unwrap().into_inner();

    match repo.get_by_email(&login.email) {
        Some(ref user) => {
            let password = argon2i_simple(&login.password, &secret.0);
            let hashed = base64::encode(&password);
            if hashed == user.password {
                let jwt = UserToken { id: user.id, name: user.name.to_owned() }.to_jwt();
                cookies.add(Cookie::new("user", jwt));
                Redirect::to("/")
            } else {
                Redirect::to("/login")
            }
        },
        _ => Redirect::to("/login"),
    }
}
