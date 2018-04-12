use rocket::request::Form;
use rocket::response::Redirect;
use rocket_contrib::Template;

use db;
use login::UserCookie;
use models::{NewPost, Post};
use schema::posts::dsl::*;
use diesel::prelude::*;
use diesel::insert_into;

#[get("/")]
fn index(db: db::Database, user_cookie: Option<UserCookie>) -> Template {
    let mut context = UserCookie::context_or(&user_cookie);

    let last_post: Vec<Post> = posts.limit(50)
        .load::<Post>(&*db)
        .expect("Error loading posts")
        .into_iter()
        .map(|p| p.to_html())
        .collect();

    context.insert("posts".to_string(), json!(last_post));
    Template::render("index", &context)
}

#[get("/<post_id>")]
fn get(db: db::Database, post_id: i32, user_cookie: Option<UserCookie>) -> Template {
    let mut context = UserCookie::context_or(&user_cookie);

    let post = posts.filter(id.eq(post_id))
        .first::<Post>(&*db)
        .expect("Error loading posts"); // TODO return 404

    context.insert("post".to_string(), json!(post.to_html()));
    Template::render("post", context)
}

#[get("/new")]
fn edit_new(user_cookie: UserCookie) -> Template {
    Template::render("edit_post", UserCookie::context(&user_cookie))
}

#[post("/new", data = "<post_form>")]
fn new(db: db::Database, _user_cookie: UserCookie, post_form: Option<Form<NewPost>>) -> Result<Redirect, Template> {
    let post = post_form.unwrap().into_inner();

    let new_post = insert_into(posts)
        .values(&post)
        .get_result::<Post>(&*db)
        .expect("Failed to insert post");

    match post.validate() {
        Ok(_)    => Ok(Redirect::to(format!("/post/{}", new_post.id).as_str())),
        Err(_) => Err(Template::render("edit_post", &new_post)),
    }
}
