use db::Database;

use schema::posts;
use schema::posts::dsl;
use diesel::prelude::*;
use diesel::insert_into;

#[derive(Queryable, Serialize, Deserialize, Clone, FromForm, Insertable)]
#[table_name="posts"]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

#[derive(FromForm, Insertable)]
#[table_name="posts"]
pub struct NewPost {
    pub title: String,
    pub body: String,
    pub published: bool,
}


// impl PostRepository {
//     fn new(db)
// }

pub trait PostRepository {
    fn insert(self, post: NewPost) -> Post;
}

impl PostRepository for Database {
    fn insert(self, post: NewPost) -> Post {
        insert_into(dsl::posts)
            .values(&post)
            .get_result::<Post>(&*self)
            .expect("Failed to insert post")
    }
}
