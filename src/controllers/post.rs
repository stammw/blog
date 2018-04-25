use rocket::request::Form;
use rocket::response::status::NotFound;
use rocket::response::Redirect;
use rocket_contrib::Template;

use login::UserCookie;
use models::NewPost;
use repositories::post::PostRepository;
use repositories::user::UserRepository;

#[get("/")]
pub fn index(
    post_repo: Box<PostRepository>,
    user_repo: Box<UserRepository>,
    user_cookie: Option<UserCookie>,
) -> Result<Template, Redirect> {
    let mut context = UserCookie::context_or(&user_cookie);

    let last_post = post_repo.all(50);

    if last_post.len() < 1 && user_cookie.is_none() {
        let user = user_repo.count();
        if user == 0 {
            return Err(Redirect::to("/create_user"));
        }
    }

    context.insert("posts".to_string(), json!(last_post));
    Ok(Template::render("index", &context))
}

#[get("/<post_id>")]
pub fn get(
    post_repo: Box<PostRepository>,
    post_id: i32,
    user_cookie: Option<UserCookie>,
) -> Result<Template, NotFound<&'static str>> {
    let mut context = UserCookie::context_or(&user_cookie);

    match post_repo.get(post_id) {
        Some(post) => {
            context.insert("post".to_string(), json!(post.to_html()));
            Ok(Template::render("post", context))
        }
        None => Err(NotFound("This article does not exists")),
    }
}

#[get("/new")]
fn edit_new(user_cookie: UserCookie) -> Template {
    Template::render("edit_post", UserCookie::context(&user_cookie))
}

#[post("/new", data = "<post_form>")]
fn new(
    post_repo: Box<PostRepository>,
    _user_cookie: UserCookie,
    post_form: Option<Form<NewPost>>,
) -> Result<Redirect, Template> {
    let post = post_form.unwrap().into_inner();

    let new_post = post_repo.insert(&post);

    match post.validate() {
        Ok(_) => Ok(Redirect::to(format!("/post/{}", new_post.id))),
        Err(_) => Err(Template::render("edit_post", &new_post)),
    }
}
