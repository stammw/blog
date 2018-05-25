use login::UserCookie;
use repositories::user::UserRepository;
use rocket::http::Status;
use rocket_contrib::Template;

#[get("/new")]
pub fn new(user_repo: Box<UserRepository>, user_cookie: Option<UserCookie>) -> Result<Template, Status>{
    match user_cookie {
        Some(_) => Ok(Template::render("create_user", vec![0])),
        None if user_repo.count() < 1 => Ok(Template::render("create_user", vec![0])),
        None => Err(Status::new(403, "You cannot create users, sorry.")),
    }
}

