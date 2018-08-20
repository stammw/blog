use db::Database;
use diesel::prelude::*;
use diesel;
use models::{Post,NewPost};
use rocket::Request;
use rocket::request::{FromRequest, Outcome};
use schema::posts::dsl::*;

pub struct PostRepositoryImpl(Database);

pub trait PostRepository {
    fn all(&self, limit: i64, published_: Option<bool>) -> Vec<Post>;
    fn all_published(&self, limit: i64) -> Vec<Post>;
    fn get(&self, post_id: i32) -> Option<Post>;
    fn get_by_slug(&self, post_slug: &str) -> Option<Post>;
    fn insert(&self, post: &NewPost) -> Post;
    fn update(&self, post: &Post) -> Post;
}

impl PostRepository for PostRepositoryImpl {
    fn all_published(&self, limit: i64) -> Vec<Post> {
         posts.limit(limit)
            .filter(published.eq(true))
            .load::<Post>(&*self.0)
            .expect("Error loading posts")
    }

    fn all(&self, limit: i64, published_: Option<bool>) -> Vec<Post> {
        let mut req = posts.limit(limit).into_boxed();

        if let Some(published_) = published_ {
            req = req.filter(published.eq(published_));
        }

        req.load::<Post>(&*self.0)
            .expect("Error loading posts")
    }

    fn get(&self, post_id: i32) -> Option<Post> {
        let result = posts.filter(id.eq(post_id))
            .first::<Post>(&*self.0);

        match result {
            Ok(p) => Some(p),
            Err(diesel::NotFound) => None,
            Err(_) => panic!("Failed to retreive one Post"),
        }
    }

    fn get_by_slug(&self, post_slug: &str) -> Option<Post> {
        let result = posts.filter(
            slug.eq(post_slug)
                .and(published.eq(true))
        ).first::<Post>(&*self.0);

        match result {
            Ok(p) => Some(p),
            Err(diesel::NotFound) => None,
            Err(_) => panic!("Failed to retreive one Post"),
        }
    }

    fn insert(&self, post: &NewPost) -> Post {
        diesel::insert_into(posts)
            .values(post)
            .get_result::<Post>(&*self.0)
            .expect("Failed to insert post")
    }

    fn update(&self, post: &Post) -> Post {
        diesel::update(posts.filter(id.eq(post.id)))
            .set(post)
            .get_result::<Post>(&*self.0)
            .expect("Failed to insert post")
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for Box<PostRepository> {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> Outcome<Box<PostRepository>, ()> {
        request.guard::<Database>()
            .map(|d| Box::new(PostRepositoryImpl(d)) as Box<PostRepository>)
    }
}
