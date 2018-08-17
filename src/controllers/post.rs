use std::collections::HashMap;
use rocket::request::Form;
use rocket::response::status::{BadRequest, NotFound};
use rocket::response::Redirect;
use rocket_contrib::Template;
use slug;

use auth::{ForwardUserToken, UserToken};
use models::NewPost;
use models::Post;
use repositories::posts::PostRepository;
use repositories::users::UserRepo;

#[get("/")]
pub fn index(post_repo: Box<PostRepository>, repo: UserRepo, user: Option<UserToken>)
         -> Result<Template, Redirect> {
    let last_post = post_repo.all(50);
    Ok(Template::render("index", json!({ "user": user, "posts": last_post })))
}

#[get("/<post_id>")]
pub fn get(post_repo: Box<PostRepository>, post_id: i32, user: ForwardUserToken)
           -> Result<Template, NotFound<&'static str>> {
    println!("get id {}", post_id);
    match post_repo.get(post_id) {
        Some(post) => {
            Ok(Template::render("post", json!({ "user": user.0, "post": post.to_html() })))
        }
        None => Err(NotFound("This article does not exists")),
    }
}

#[get("/<slug>", rank = 2)]
pub fn get_by_slug(post_repo: Box<PostRepository>, slug: String, user: Option<UserToken>)
                   -> Result<Template, NotFound<&'static str>> {
    match post_repo.get_by_slug(&slug) {
        Some(post) => {
            Ok(Template::render("post", json!({ "user": user, "post": post.to_html() })))
        }
        None => Err(NotFound("This article does not exists")),
    }
}

#[get("/new")]
fn edit_new(_user: UserToken) -> Template {
    Template::render("edit_post", ())
}

#[derive(FromForm, Serialize)]
pub struct PostForm {
    pub title: String,
    pub body: String,
    pub published: bool,
}

impl PostForm {
    pub fn validate(&self) -> Result<&PostForm, HashMap<&'static str, &'static str>> {
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
            title: self.title.to_owned(),
            slug: slug::slugify(&self.title),
            body: self.body.to_owned(),
            published: self.published,
        }
    }

    pub fn to_model(&self, id: i32) -> Post {
        Post {
            id: id,
            title: self.title.to_owned(),
            slug: slug::slugify(&self.title),
            body: self.body.to_owned(),
            published: self.published,
        }
    }
}

#[post("/new", data = "<form>")]
fn new(post_repo: Box<PostRepository>, user: UserToken, form: Form<PostForm>)
       -> Result<Redirect, BadRequest<Template>> {
    let post = form.into_inner();

    match post.validate() {
        Ok(p) => {
            let insertable = &p.to_insertable();
            let new_post = post_repo.insert(insertable);
            if new_post.published {
                Ok(Redirect::to(&format!("/post/{}", new_post.slug)))
            } else {
                Ok(Redirect::to(&format!("/post/{}", new_post.id)))
            }
        },
        Err(_) => Err(BadRequest(Some(Template::render(
            "edit_post", json!({ "user": user, "post": &post})
        )))),
    }
}

#[get("/<post_id>/edit")]
fn edit(post_id: i32, user: UserToken, post_repo: Box<PostRepository>)
            -> Result<Template, NotFound<&'static str>> {
    match post_repo.get(post_id) {
        Some(post) => {
            Ok(Template::render("edit_post", json!({ "user": user, "post": post })))
        }
        None => Err(NotFound("This article does not exists")),
    }
}

#[post("/<post_id>", data = "<form>")]
pub fn update(post_repo: Box<PostRepository>, post_id: i32, form: Form<PostForm>, user: UserToken)
           -> Result<Redirect, BadRequest<Template>> {
    let post = form.into_inner();

    match post.validate() {
        Ok(p) => {
            let post = p.to_model(post_id);
            let post = post_repo.update(&post);
            if post.published {
                Ok(Redirect::to(&format!("/post/{}", post.slug)))
            } else {
                Ok(Redirect::to(&format!("/post/{}", post.id)))
            }
        },
        Err(_) => Err(BadRequest(Some(Template::render(
            "edit_post", json!({ "user": user, "post": &post})
        )))),
    }
}
