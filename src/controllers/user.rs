use auth::UserToken;
use models::NewUser;
use repositories::users::UserRepo;
use rocket::response::status::BadRequest;
use rocket::request::Form;
use rocket_contrib::Template;

#[get("/new")]
pub fn new(_cookie: UserToken) -> Result<Template, BadRequest<String>> {
    Ok(Template::render("new", ()))
}

#[post("/create", data = "<user_form>")]
pub fn create(user_form: Form<NewUser>, repo: UserRepo, _cookie: UserToken)
              -> Result<Template, BadRequest<Template>> {
    let user = user_form.get();

    if let Err(e) = user.validate() {
        return Err(BadRequest(Some(Template::render("user_new", json!({ "error": e })))));
    }

    if repo.get_by_email_or_name(&user.email, &user.name).is_some() {
        return Err(BadRequest(Some(Template::render("user_new", json!({"error":  "user already exists"})))));
    }

    Ok(Template::render("user_new", ()))
}
