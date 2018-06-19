use login::UserCookie;
use models::NewUser;
use repositories::users::UserRepository;
use rocket::http::Status;
use rocket::request::Form;
use rocket_contrib::Template;

fn check_access(repo: &UserRepository, cookie: &Option<UserCookie>) -> Result<(), Status> {
    match cookie {
        Some(_) => Ok(()),
        None if repo.count() < 1 => Ok(()),
        None => Err(Status::new(403, "You cannot create users, sorry.")),
    }
}

#[get("/new")]
pub fn new(repo: UserRepository, cookie: Option<UserCookie>) -> Result<Template, Status> {
    check_access(&repo, &cookie)?;
    Ok(Template::render("new", UserCookie::context_or(&cookie)))
}

#[post("/create", data = "<user_form>")]
pub fn create(
    user_form: Form<NewUser>,
    repo: UserRepository,
    cookie: Option<UserCookie>
) -> Result<Template, Status> {
    check_access(&repo, &cookie)?;
    let user = user_form.get();
    if repo.get_by_name(&user.name).is_some() {
        return Err(Status::raw(400));
    }
    Ok(Template::render("user_new", UserCookie::context_or(&cookie)))
}
