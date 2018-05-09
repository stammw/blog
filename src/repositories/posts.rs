use db::Database;
use diesel::prelude::*;
use diesel;
use models::{Post,NewPost};
use rocket::Request;
use rocket::request::{FromRequest, Outcome};
use schema::posts::dsl::*;

pub struct PostRepositoryImpl(Database);

pub trait PostRepository {
    fn all(&self, limit: i64) -> Vec<Post>;
    fn get(&self, post_id: i32) -> Option<Post>;
    fn insert(&self, post: &NewPost) -> Post;
}

impl PostRepository for PostRepositoryImpl {
    fn all(&self, limit: i64) -> Vec<Post> {
         posts.limit(limit)
            .load::<Post>(&*self.0)
            .expect("Error loading posts")
            .into_iter()
            .map(|p| p.to_html())
            .collect()
    }

    fn get(&self, post_id: i32) -> Option<Post> {
        let result = posts.filter(id.eq(post_id))
            .first::<Post>(&*self.0);

        match result {
            Ok(p) => Some(p),
            Err(diesel::NotFound) => None,
            Err(e) => panic!("Failed to retreive one Post"),
        }
    }

    fn insert(&self, post: &NewPost) -> Post {
        diesel::insert_into(posts)
            .values(post)
            .get_result::<Post>(&*self.0)
            .expect("Failed to insert post")
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for Box<PostRepository> {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> Outcome<Box<PostRepository>, ()> {
        Database::from_request(request)
            .map(|d| Box::new(PostRepositoryImpl(d)) as Box<PostRepository>)
    }
}
