use rocket::request::Form;
use rocket::response::status::{BadRequest, NotFound};
use rocket::response::Redirect;
use rocket_contrib::Template;

use auth::UserToken;
use models::NewPost;
use repositories::posts::PostRepository;
use repositories::users::UserRepo;

#[get("/")]
pub fn index(post_repo: Box<PostRepository>, _repo: UserRepo, _user: Option<UserToken>)
         -> Result<Template, Redirect> {
    let last_post = post_repo.all(50);
    Ok(Template::render("index", json!({ "posts": last_post })))
}

#[get("/<post_id>")]
pub fn get(post_repo: Box<PostRepository>, post_id: i32) -> Result<Template, NotFound<&'static str>> {
    match post_repo.get(post_id) {
        Some(post) => {
            Ok(Template::render("post", json!({ "post": post.to_html() })))
        }
        None => Err(NotFound("This article does not exists")),
    }
}

#[get("/new")]
fn edit_new(_user: UserToken) -> Template {
    Template::render("edit_post", ())
}

#[post("/new", data = "<post_form>")]
fn new(post_repo: Box<PostRepository>, _user: UserToken, post_form: Option<Form<NewPost>>)
       -> Result<Redirect, BadRequest<Template>> {
    let post = post_form.unwrap().into_inner();

    let new_post = post_repo.insert(&post);

    match post.validate() {
        Ok(_) => Ok(Redirect::to(format!("/post/{}", new_post.id).as_str())),
        Err(_) => Err(BadRequest(Some(Template::render("edit_post", &new_post)))),
    }
}
