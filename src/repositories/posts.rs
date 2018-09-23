use std::collections::HashMap;
use db::Database;
use diesel::prelude::*;
use diesel::dsl::*;
use diesel;
use models::{Post,NewPost, User, Comment};
use rocket::Request;
use rocket::request::{FromRequest, Outcome};
use schema::posts::dsl::*;
use schema::users;

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
    fn all_published(&self, limit: i64, page: i64) -> Vec<(User, Post, Vec<Comment>)>;
    fn get(&self, post_id: i32) -> Option<Post>;
    fn get_by_slug(&self, post_slug: &str) -> Option<Post>;
    fn insert(&self, post: &NewPost) -> Post;
    fn update(&self, post: &Post) -> Post;
    fn count(&self) -> i64;
    fn is_published(&self, post_id: i32) -> bool;
}

impl PostRepoTrait for PostRepoImpl {
    fn all_published(&self, limit: i64, page: i64) -> Vec<(User, Post, Vec<Comment>)> {
        let users_map = users::table.load::<User>(&*self.db)
            .expect("Error loading users")
            .into_iter()
            .map(|u| (u.id, u))
            .collect::<HashMap<i32, User>>();

        let published_posts = posts.limit(limit).offset(limit * page)
            .filter(published.eq(true))
            .order(publication_date.desc())
            .load::<Post>(&*self.db)
            .expect("Error loading posts");

        let comments = Comment::belonging_to(&published_posts)
            .load::<Comment>(&*self.db).expect("Failed to load comments")
            .grouped_by(&published_posts);

        published_posts
            .into_iter()
            .zip(comments)
            .map(|(p, c)| (users_map.get(&p.user_id).unwrap().clone(), p, c))
            .collect()
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

    fn count(&self) -> i64 {
        posts.select(count(id))
            .filter(published.eq(true))
            .first(&*self.db)
            .expect("Could not count posts")
    }

    fn is_published(&self, post_id: i32) -> bool {
        let count: i64 = posts.select(count(id))
            .filter(id.eq(post_id).and(published.eq(true)))
            .first(&*self.db)
            .expect("Could not count posts");
        count > 0
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for PostRepo {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> Outcome<PostRepo, ()> {
        request.guard::<Database>()
            .map(|db| factory(db))
    }
}
