use rocket::{get, post};
use rocket::response::status::BadRequest;
use rocket::request::Form;
use rocket_contrib::templates::Template;
use serde_json::map::Map;

use auth::UserToken;
use db::Database;
use models::{NewUser, User};

#[get("/new")]
pub fn new(token: UserToken) -> Result<Template, BadRequest<String>> {
    Ok(Template::render("user/new", json!({ "user": token })))
}

#[post("/create", data = "<user_form>")]
pub fn create(user_form: Form<NewUser>, db: Database, token: UserToken)
              -> Result<Template, BadRequest<Template>> {
    let form = user_form.into_inner();

    if let Err(e) = form.validate() {
        return Err(BadRequest(Some(Template::render("user/new", json!({ "user": token, "error": e })))));
    }

    if let Some(user) = User::get_by_email_or_name(&db, &form.email, &form.name) {
        let mut errors = Map::new();
        if form.name == user.name {
           errors.entry("name").or_insert(json!("This user name already exists").0);
        }

        if form.email == user.email {
           errors.entry("email").or_insert(json!("This email already exists").0);
        }

        return Err(BadRequest(Some(Template::render("user/new", json!({ "user": token, "error": errors})))));
    }

    Ok(Template::render("user/new", ()))
}
