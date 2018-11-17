use std::collections::HashMap;
use chrono::Utc;
use rocket::{get, post};
use rocket::request::Form;
use rocket::response::status::{BadRequest, NotFound};
use rocket::http::Status;
use rocket::response::Redirect;
use rocket_contrib::templates::Template;
use rocket_contrib::json::JsonValue;
use slug;

use auth::{ForwardUserToken, UserToken};
use db::Database;
use models::{Post, NewPost};
use pagination::PaginationParams;

fn index_pagination(pagination: PaginationParams, db: &Database, user: Option<UserToken>)
         -> Result<Template, Redirect> {
    let last_posts: Vec<JsonValue> = Post::all_published(db, 5, pagination.page as i64 - 1)
        .into_iter()
        .map(|(u, p)| {
            let mut post = json!(p.to_html());
            {
                let post_json = post.as_object_mut().unwrap();
                post_json.insert("user".to_string(), json!(u).0);
            }
            post
        }).collect();

    Ok(Template::render("index", json!({
        "user": user,
        "posts": last_posts,
        "pages": pagination.context(Post::count(db) as u32),
    })))
}

#[get("/")]
pub fn index(db: Database, user: Option<UserToken>)
         -> Result<Template, Redirect> {
    index_pagination(PaginationParams::default(), &db, user)
}

#[get("/?<pagination..>")]
pub fn index_page(pagination: Form<PaginationParams>, db: Database, user: Option<UserToken>)
         -> Result<Template, Redirect> {
    index_pagination(pagination.into_inner(), &db, user)
}

#[derive(FromForm, Serialize)]
pub struct ListParams {
    pub published: Option<bool>,
}

impl Default for ListParams {
    fn default() -> ListParams {
        ListParams {
            published: None,
        }
    }
}

fn list_params(params: ListParams, user: UserToken, db: &Database)
            -> Result<Template, NotFound<&'static str>> {
    let posts = Post::all(db, 50, params.published);
    Ok(Template::render("post/list", json!({ "user": user, "posts": posts, "params": params })))
}

#[get("/list")]
pub fn list(user: UserToken, db: Database)
            -> Result<Template, NotFound<&'static str>> {
    list_params(ListParams::default(), user, &db)
}

#[get("/list?<params..>")]
pub fn list_with_params(params: Form<ListParams>, user: UserToken, db: Database)
            -> Result<Template, NotFound<&'static str>> {
    list_params(params.into_inner(), user, &db)
}


#[get("/<post_id>")]
pub fn get(db: Database, post_id: i32, user: ForwardUserToken)
           -> Result<Template, diesel::result::Error> {
    println!("get id {}", post_id);
    Ok(Template::render("post/display", json!({
        "user": user.0,
        "post": Post::get(&db, post_id)?.to_html()
    })))
}

#[get("/<slug>", rank = 2)]
pub fn get_by_slug(db: Database, slug: String, user: Option<UserToken>)
                   -> Result<Template, rocket::http::Status> {
    match Post::get_by_slug(&db, &slug) {
        Ok((post, author, comments)) => {
            let context = json!({
                "user": user,
                "post": post.to_html(),
                "author": author,
                "comments": comments.into_iter().map(|(comment, author)| {
                    json!({ "comment": comment, "author": author }).0
                }).collect::<serde_json::Value>(),
            });
            Ok(Template::render("post/display", context))
        }
        Err(diesel::NotFound) => Err(Status::NotFound),
        Err(_) => Err(Status::new(500, "Internal server error")),
    }
}

#[get("/new")]
pub fn edit_new(_user: UserToken) -> Template {
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
pub fn new(db: Database, user: UserToken, form: Form<PostForm>)
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

    let inserted = Post::insert(&db, &post);
    if inserted.published {
        Ok(Redirect::to(uri!(get_by_slug: inserted.slug)))
    } else {
        Ok(Redirect::to(uri!(get: inserted.id)))
    }
}

#[get("/<post_id>/edit")]
pub fn edit(post_id: i32, user: UserToken, db: Database)
            -> Result<Template, NotFound<&'static str>> {
    match Post::get(&db, post_id) {
        Ok(post) => {
            Ok(Template::render("post/edit", json!({ "user": user, "post": post })))
        }
        Err(_) => Err(NotFound("This article does not exists")),
    }
}

#[post("/<post_id>", data = "<form>")]
pub fn update(db: Database, post_id: i32, form: Form<PostForm>, user: UserToken)
           -> Result<Redirect, BadRequest<Template>> {
    let post = form.into_inner();

    if let Err(_e) = post.validate() {
        return Err(BadRequest(Some(Template::render(
            "post/edit", json!({ "user": user, "post": &post})
        ))))
    }

    let existing_post = match Post::get(&db, post_id) {
        Ok(p) => p,
        Err(_)    => return Err(BadRequest(Some(Template::render( // TODO should be not found
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

    let post = Post::update(&db, &post);
    if post.published {
        Ok(Redirect::to(uri!(get_by_slug: post.slug)))
    } else {
        Ok(Redirect::to(uri!(get: post.id)))
    }
}
