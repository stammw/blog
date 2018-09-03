use db::Database;
use diesel::prelude::*;
use diesel;
use models::{Post,NewPost};
use rocket::Request;
use rocket::request::{FromRequest, Outcome};
use schema::posts::dsl::*;

pub type PostRepo = Box<PostRepoTrait + Send>;

pub type PostRepoFactory = fn(db: Database) -> PostRepo;

pub struct PostRepoImpl {
  pub db: Database,
}

pub fn factory(db: Database) -> PostRepo {
    Box::new(PostRepoImpl { db: db }) as PostRepo
}

pub trait PostRepoTrait {
    fn all(&self, limit: i64, published_: Option<bool>) -> Vec<Post>;
    fn all_published(&self, limit: i64) -> Vec<Post>;
    fn get(&self, post_id: i32) -> Option<Post>;
    fn get_by_slug(&self, post_slug: &str) -> Option<Post>;
    fn insert(&self, post: &NewPost) -> Post;
    fn update(&self, post: &Post) -> Post;
}

impl PostRepoTrait for PostRepoImpl {
    fn all_published(&self, limit: i64) -> Vec<Post> {
         posts.limit(limit)
            .filter(published.eq(true))
            .order(publication_date.desc())
            .load::<Post>(&*self.db)
            .expect("Error loading posts")
    }

    fn all(&self, limit: i64, published_: Option<bool>) -> Vec<Post> {
        let mut req = posts.limit(limit).into_boxed();

        if let Some(published_) = published_ {
            req = req.filter(published.eq(published_));
        }

        req.load::<Post>(&*self.db)
            .expect("Error loading posts")
    }

    fn get(&self, post_id: i32) -> Option<Post> {
        let result = posts.filter(id.eq(post_id))
            .first::<Post>(&*self.db);

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
        ).first::<Post>(&*self.db);

        match result {
            Ok(p) => Some(p),
            Err(diesel::NotFound) => None,
            Err(_) => panic!("Failed to retreive one Post"),
        }
    }

    fn insert(&self, post: &NewPost) -> Post {
        diesel::insert_into(posts)
            .values(post)
            .get_result::<Post>(&*self.db)
            .expect("Failed to insert post")
    }

    fn update(&self, post: &Post) -> Post {
        diesel::update(posts.filter(id.eq(post.id)))
            .set(post)
            .get_result::<Post>(&*self.db)
            .expect("Failed to insert post")
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for PostRepo {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> Outcome<PostRepo, ()> {
        request.guard::<Database>()
            .map(|db| factory(db))
    }
}
