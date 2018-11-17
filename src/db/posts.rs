use std::collections::HashMap;
use db::{Database, Result};
use diesel::prelude::*;
use diesel::dsl::*;
use diesel;
use models::{Post, NewPost, User, Comment};
use schema::posts::dsl::*;
use schema::users;

impl Post {
    pub fn all_published(conn: &Database, limit: i64, page: i64) -> Vec<(User, Post)> {
        let users_map = users::table.load::<User>(&conn.0)
            .expect("Error loading users")
            .into_iter()
            .map(|u| (u.id, u))
            .collect::<HashMap<i32, User>>();

        let published_posts = posts.limit(limit).offset(limit * page)
            .filter(published.eq(true))
            .order(publication_date.desc())
            .load::<Post>(&conn.0)
            .expect("Error loading posts");

        published_posts
            .into_iter()
            .map(|p| (users_map.get(&p.user_id).unwrap().clone(), p))
            .collect()
    }

    pub fn all(conn: &Database, limit: i64, published_: Option<bool>) -> Vec<Post> {
        let mut req = posts.limit(limit).into_boxed();

        if let Some(published_) = published_ {
            req = req.filter(published.eq(published_));
        }

        req.load::<Post>(&conn.0)
            .expect("Error loading posts")
    }

    pub fn get(conn: &Database, post_id: i32) -> Result<Post> {
        posts.filter(id.eq(post_id))
            .first::<Post>(&conn.0)
    }

    pub fn get_by_slug(conn: &Database, post_slug: &str) -> Result<(Post, User, Vec<(Comment, User)>)> {
        let users_map = users::table.load::<User>(&conn.0)
            .expect("Error loading users")
            .into_iter()
            .map(|u| (u.id, u))
            .collect::<HashMap<i32, User>>();

        let post = posts.filter(
            slug.eq(post_slug)
                .and(published.eq(true))
        ).first::<Post>(&conn.0)?;

        let post_author = users_map.get(&post.user_id).expect("This post has no authors.");
        let comments = Comment::belonging_to(&post)
            .load::<Comment>(&conn.0)?;

        let comments_and_authors = comments.into_iter().filter_map(|c| {
            if let Some(comment_author) = users_map.get(&c.user_id) {
                Some((c, comment_author.to_owned()))
            } else {
                None
            }
        }).collect();// TODO Join comments and users
        Ok((post, post_author.to_owned(), comments_and_authors))
    }

    pub fn insert(conn: &Database, post: &NewPost) -> Post {
        diesel::insert_into(posts)
            .values(post)
            .get_result::<Post>(&conn.0)
            .expect("Failed to insert post")
    }

    pub fn update(conn: &Database, post: &Post) -> Post {
        diesel::update(posts.filter(id.eq(post.id)))
            .set(post)
            .get_result::<Post>(&conn.0)
            .expect("Failed to insert post")
    }

    pub fn count(conn: &Database) -> i64 {
        posts.select(count(id))
            .filter(published.eq(true))
            .first(&conn.0)
            .expect("Could not count posts")
    }

    pub fn is_published(conn: &Database, post_id: i32) -> bool {
        let count: i64 = posts.select(count(id))
            .filter(id.eq(post_id).and(published.eq(true)))
            .first(&conn.0)
            .expect("Could not count posts");
        count > 0
    }
}
