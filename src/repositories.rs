use db::Database;
use models::{Post,NewPost};
use rocket::Request;
use rocket::request::{FromRequest, Outcome};
use diesel::prelude::*;
use diesel::insert_into;
use schema::posts::dsl::*;

pub struct PostRepository(Database);

impl PostRepository {
    pub fn all(&self, limit: i64) -> Vec<Post> {
         posts.limit(limit)
            .load::<Post>(&*self.0)
            .expect("Error loading posts")
            .into_iter()
            .map(|p| p.to_html())
            .collect()
    }

    pub fn get(&self, post_id: i32) -> Post {
        posts.filter(id.eq(post_id))
            .first::<Post>(&*self.0)
            .expect("Error loading posts")
    }

    pub fn insert(&self, post: &NewPost) -> Post {
        insert_into(posts)
            .values(post)
            .get_result::<Post>(&*self.0)
            .expect("Failed to insert post")
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for PostRepository {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> Outcome<PostRepository, ()> {
        Database::from_request(request).map(|d| PostRepository(d))
    }
}
