use db::Database;
use diesel::prelude::*;
use diesel;
use models::{Comment, NewComment};
use rocket::Request;
use rocket::request::{FromRequest, Outcome};
use schema::comments::dsl::*;

pub type CommentRepo = Box<CommentRepoTrait + Send>;

pub type CommentRepoFactory = fn(db: Database) -> CommentRepo;

pub struct CommentRepoImpl {
  pub db: Database,
}

pub fn factory(db: Database) -> CommentRepo {
    Box::new(CommentRepoImpl { db: db }) as CommentRepo
}

pub trait CommentRepoTrait {
    fn insert(&self, comment: &NewComment) -> Comment;
}

impl CommentRepoTrait for CommentRepoImpl {
    fn insert(&self, comment: &NewComment) -> Comment {
        diesel::insert_into(comments)
            .values(comment)
            .get_result::<Comment>(&*self.db)
            .expect("Failed to insert comment")
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for CommentRepo {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> Outcome<CommentRepo, ()> {
        request.guard::<Database>()
            .map(|db| factory(db))
    }
}
