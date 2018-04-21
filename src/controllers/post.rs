use rocket::request::Form;
use rocket::response::Redirect;
use rocket_contrib::Template;

use db;
use login::UserCookie;
use models::NewPost;
use schema::users::dsl::{users};
use diesel::prelude::*;
use repositories::PostRepository;

#[get("/")]
fn index(post_repo: PostRepository, db: db::Database, user_cookie: Option<UserCookie>) -> Result<Template, Redirect> {
    let mut context = UserCookie::context_or(&user_cookie);

    let last_post = post_repo.all(50);
    if last_post.len() < 1 && user_cookie.is_none() {
        let user: i64 = users.count().get_result(&*db).unwrap();
        if user == 0 {
            return Err(Redirect::to("/create_user"));
        }
    }

    context.insert("posts".to_string(), json!(last_post));
    Ok(Template::render("index", &context))
}

#[get("/<post_id>")]
fn get(post_repo: PostRepository, post_id: i32, user_cookie: Option<UserCookie>) -> Template {
    let mut context = UserCookie::context_or(&user_cookie);

    let post = post_repo.get(post_id);

    context.insert("post".to_string(), json!(post.to_html()));
    Template::render("post", context)
}

#[get("/new")]
fn edit_new(user_cookie: UserCookie) -> Template {
    Template::render("edit_post", UserCookie::context(&user_cookie))
}

#[post("/new", data = "<post_form>")]
fn new(post_repo: PostRepository, _user_cookie: UserCookie, post_form: Option<Form<NewPost>>) -> Result<Redirect, Template> {
    let post = post_form.unwrap().into_inner();

    let new_post = post_repo.insert(&post);

    match post.validate() {
        Ok(_)    => Ok(Redirect::to(format!("/post/{}", new_post.id))),
        Err(_) => Err(Template::render("edit_post", &new_post)),
    }
}
