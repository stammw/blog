use std::collections::HashMap;
use chrono::Utc;
use rocket::request::Form;
use rocket::response::status::{BadRequest, NotFound};
use rocket::response::Redirect;
use rocket_contrib::Template;
use slug;

use auth::{ForwardUserToken, UserToken};
use models::NewPost;
use models::Post;
use pagination::PaginationParams;
use repositories::posts::PostRepo;

#[get("/")]
pub fn index(post_repo: PostRepo, user: Option<UserToken>)
         -> Result<Template, Redirect> {
    index_page(PaginationParams::default(), post_repo, user)
}

#[get("/?<pagination>")]
pub fn index_page(pagination: PaginationParams, post_repo: PostRepo, user: Option<UserToken>)
         -> Result<Template, Redirect> {
    let last_post: Vec<Post> = post_repo.all_published(5, pagination.page as i64 - 1)
        .into_iter()
        .map(|p| p.to_html())
        .collect();

    Ok(Template::render("index", json!({
        "user": user,
        "posts": last_post,
        "pages": pagination.context(post_repo.count() as u32),
    })))
}

#[derive(FromForm, Serialize)]
struct ListParams {
    pub published: Option<bool>,
}

impl Default for ListParams {
    fn default() -> ListParams {
        ListParams {
            published: None,
        }
    }
}

#[get("/list")]
fn list(user: UserToken, post_repo: PostRepo)
            -> Result<Template, NotFound<&'static str>> {
    list_params(ListParams::default(), user, post_repo)
}

#[get("/list?<params>")]
fn list_params(params: ListParams, user: UserToken, post_repo: PostRepo)
            -> Result<Template, NotFound<&'static str>> {
    let posts = post_repo.all(50, params.published);
    Ok(Template::render("post/list", json!({ "user": user, "posts": posts, "params": params })))
}

#[get("/<post_id>")]
pub fn get(post_repo: PostRepo, post_id: i32, user: ForwardUserToken)
           -> Result<Template, NotFound<&'static str>> {
    println!("get id {}", post_id);
    match post_repo.get(post_id) {
        Some(post) => {
            Ok(Template::render("post/display", json!({ "user": user.0, "post": post.to_html() })))
        }
        None => Err(NotFound("This article does not exists")),
    }
}

#[get("/<slug>", rank = 2)]
pub fn get_by_slug(post_repo: PostRepo, slug: String, user: Option<UserToken>)
                   -> Result<Template, NotFound<&'static str>> {
    match post_repo.get_by_slug(&slug) {
        Some(post) => {
            Ok(Template::render("post/display", json!({ "user": user, "post": post.to_html() })))
        }
        None => Err(NotFound("This article does not exists")),
    }
}

#[get("/new")]
fn edit_new(_user: UserToken) -> Template {
    Template::render("post/edit", ())
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
}

#[post("/new", data = "<form>")]
fn new(post_repo: PostRepo, user: UserToken, form: Form<PostForm>)
       -> Result<Redirect, BadRequest<Template>> {
    let post = form.into_inner();

    if let Err(_) = post.validate() {
        return Err(BadRequest(Some(Template::render(
            "post/edit", json!({ "user": user, "post": &post})
        ))));
    }

    let post = NewPost {
        user_id: user.id,
        title: post.title.to_owned(),
        slug: slug::slugify(&post.title),
        body: post.body.to_owned(),
        published: post.published,
        creation_date: Utc::now().naive_utc(),
        publication_date: match post.published {
            true  => Some(Utc::now().naive_utc()),
            false => None,
        },
    };

    let inserted = post_repo.insert(&post);
    if inserted.published {
        Ok(Redirect::to(&format!("/post/{}", inserted.slug)))
    } else {
        Ok(Redirect::to(&format!("/post/{}", inserted.id)))
    }
}

#[get("/<post_id>/edit")]
fn edit(post_id: i32, user: UserToken, post_repo: PostRepo)
            -> Result<Template, NotFound<&'static str>> {
    match post_repo.get(post_id) {
        Some(post) => {
            Ok(Template::render("post/edit", json!({ "user": user, "post": post })))
        }
        None => Err(NotFound("This article does not exists")),
    }
}

#[post("/<post_id>", data = "<form>")]
pub fn update(post_repo: PostRepo, post_id: i32, form: Form<PostForm>, user: UserToken)
           -> Result<Redirect, BadRequest<Template>> {
    let post = form.into_inner();

    if let Err(_e) = post.validate() {
        return Err(BadRequest(Some(Template::render(
            "post/edit", json!({ "user": user, "post": &post})
        ))))
    }

    let existing_post = match post_repo.get(post_id) {
        Some(p) => p,
        None    => return Err(BadRequest(Some(Template::render(
            "post/edit", json!({ "user": user, "post": &post})
        )))),
    };

    let post = Post {
        id: post_id,
        user_id: user.id,
        slug: slug::slugify(post.title.to_owned()),
        title: post.title,
        body: post.body,
        creation_date: existing_post.creation_date,
        edition_date: Some(Utc::now().naive_utc()),
        publication_date: match (existing_post.publication_date, post.published) {
            (Some(date), _)       => Some(date),
            (None,       true)    => Some(Utc::now().naive_utc()),
            (None,       false)   => None,
        },
        published: post.published,
    };

    let post = post_repo.update(&post);
    if post.published {
        Ok(Redirect::to(&format!("/post/{}", post.slug)))
    } else {
        Ok(Redirect::to(&format!("/post/{}", post.id)))
    }
}
