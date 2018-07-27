use std::collections::HashMap;
use rocket::request::Form;
use rocket::response::status::{BadRequest, NotFound};
use rocket::response::Redirect;
use rocket_contrib::Template;
use slug;

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

#[get("/<slug>", rank = 2)]
pub fn get_by_slug(post_repo: Box<PostRepository>, slug: String) -> Result<Template, NotFound<&'static str>> {
    match post_repo.get_by_slug(&slug) {
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

#[derive(FromForm, Serialize)]
pub struct NewPostForm {
    pub title: String,
    pub body: String,
}

impl NewPostForm {
    pub fn validate(&self) -> Result<&NewPostForm, HashMap<&'static str, &'static str>> {
        let mut errors = HashMap::new();
        if self.title.is_empty() {
            errors.insert("title", "Title shall not be emty");
        }
        if self.body.is_empty() {
            errors.insert("body", "Body shall not be emty");
        }
        if ! errors.is_empty() {
           Err(errors) 
        } else {
            Ok(self)
        }
    }

    pub fn to_insertable(&self) -> NewPost {
        NewPost {
            slug: slug::slugify(&self.title),   
            title: self.title.to_owned(),
            body: self.body.to_owned(),
            published: false,
        }
    }
}


#[post("/new", data = "<form>")]
fn new(post_repo: Box<PostRepository>, _user: UserToken, form: Form<NewPostForm>)
       -> Result<Redirect, BadRequest<Template>> {
    let post = form.into_inner();

    match post.validate() {
        Ok(p) => {
            let new_post = post_repo.insert(&p.to_insertable());
            Ok(Redirect::to(format!("/post/{}", new_post.slug).as_str()))
        },
        Err(_) => Err(BadRequest(Some(Template::render("edit_post", &post)))),
    }
}
