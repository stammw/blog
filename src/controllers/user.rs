use login::UserCookie;
use models::NewUser;
use repositories::users::UserRepo;
use rocket::response::status::BadRequest;
use rocket::request::Form;
use rocket_contrib::Template;

fn check_access(repo: &UserRepo, cookie: &Option<UserCookie>) -> Result<(), String> {
    match cookie {
        Some(_) => Ok(()),
        None if repo.count() < 1 => Ok(()),
        None => Err("You cannot create users, sorry.".to_string()),
    }
}

#[get("/new")]
pub fn new(repo: UserRepo, cookie: Option<UserCookie>) -> Result<Template, BadRequest<String>> {
    match check_access(&repo, &cookie) {
        Err(_) => {
            return Err(BadRequest(None));
        },
        _ => (),
    }
    Ok(Template::render("new", UserCookie::context_or(&cookie)))
}

#[post("/create", data = "<user_form>")]
pub fn create(user_form: Form<NewUser>, repo: UserRepo, cookie: Option<UserCookie>)
              -> Result<Template, BadRequest<Template>> {
    if let Err(_) = check_access(&repo, &cookie) {
        return Err(BadRequest(None));
    }

    let mut context = UserCookie::context_or(&cookie);
    let user = user_form.get();

    if let Err(e) = user.validate() {
        context.insert("error".into(), json!(e));
        let template = Template::render("user_new", context);
        return Err(BadRequest(Some(template)));
    }

    if repo.get_by_email(&user.email).is_some() {
        context.insert("error".into(), "user already exists".into());
        let template = Template::render("user_new", context);
        return Err(BadRequest(Some(template)));
    }

    Ok(Template::render("user_new", UserCookie::context_or(&cookie)))
}
