use std::collections::HashMap;
use regex::Regex;
use pulldown_cmark::{html, Parser};
use chrono::NaiveDateTime;

use schema::{posts, users, comments};

#[derive(Queryable, Serialize, Deserialize, Clone, Insertable, Debug, AsChangeset, Identifiable, Associations)]
#[belongs_to(User)]
#[table_name = "posts"]
pub struct Post {
    pub id: i32,
    pub user_id: i32,
    pub slug: String,
    pub title: String,
    pub body: String,
    pub published: bool,
    pub creation_date: NaiveDateTime,
    pub edition_date: Option<NaiveDateTime>,
    pub publication_date: Option<NaiveDateTime>,
}

#[derive(Insertable, Debug)]
#[table_name = "posts"]
pub struct NewPost {
    pub user_id: i32,
    pub title: String,
    pub slug: String,
    pub body: String,
    pub published: bool,
    pub creation_date: NaiveDateTime,
    pub publication_date: Option<NaiveDateTime>,
}

impl Post {
    pub fn to_html(mut self) -> Post {
        let to_format = self.body.to_owned();
        let parser = Parser::new(&to_format.as_str());
        self.body.truncate(0);
        html::push_html(&mut self.body, parser);
        self
    }
}

#[derive(Identifiable, Queryable, Serialize, Deserialize, Clone, FromForm, Debug)]
pub struct User {
    pub id: i32,
    pub name: String,
    #[serde(skip_serializing)]
    pub email: String,
    #[serde(skip_serializing)]
    pub password: String,
}

#[derive(Insertable, FromForm, Clone, Debug)]
#[table_name = "users"]
pub struct NewUser {
    pub name: String,
    pub email: String,
    pub password: String,
}

impl NewUser {
    pub fn validate(&self) -> Result<(), HashMap<&'static str, &'static str>> {
        lazy_static! {
            static ref EMAIL_REGEX: Regex = Regex::new(r"\w+@\w+\.\w+").unwrap();
        }

        let mut errors = HashMap::new();
        if self.name.len() < 4 {
            errors.insert("name", "Name shall not be empty");
        }
        if !EMAIL_REGEX.is_match(&self.email) {
            errors.insert("email", "Email is not valid");
        }
        if self.password.len() < 8 {
            errors.insert("password", "Password should be at least 8 chars");
        }
        if ! errors.is_empty() {
           Err(errors) 
        } else {
            Ok(())
        }
    }
}

#[derive(Queryable, Serialize, Deserialize, Clone, Insertable, Debug, AsChangeset, Identifiable, Associations)]
#[belongs_to(Post, User)]
#[table_name = "comments"]
pub struct Comment {
    pub id: i32,
    pub user_id: i32,
    pub post_id: i32,
    pub body: String,
    pub creation_date: NaiveDateTime,
}

#[derive(Insertable, Debug)]
#[table_name = "comments"]
pub struct NewComment {
    pub user_id: i32,
    pub post_id: i32,
    pub body: String,
    pub creation_date: NaiveDateTime,
}

#[cfg(test)]
mod tests {
    use super::*;
    fn new_user(name: &'static str,  email: &'static str, password: &'static str)
                -> NewUser {
        NewUser {
            name: String::from(name),
            email: String::from(email),
            password: String::from(password),
        }
    }

    #[test]
    fn new_user_name_is_at_least_4_char() {
        let user = new_user("dat", "email@yep.yep", "password");
        let result = user.validate();
        assert!(result.is_err());
        assert_eq!(*result.unwrap_err().get("name").unwrap(),
                   "Name shall not be empty");
    }

    #[test]
    fn new_user_email_shall_be_valid() {
        let user = new_user("name", "email@yep", "password");
        let result = user.validate();
        assert!(result.is_err());
        assert_eq!(*result.unwrap_err().get("email").unwrap(),
                   "Email is not valid");
    }
    
    #[test]
    fn new_user_password_is_at_least_8_chars() {
        let user = new_user("name", "email@yep.com", "pas");
        let result = user.validate();
        assert!(result.is_err());
        assert_eq!(*result.unwrap_err().get("password").unwrap(),
                   "Password should be at least 8 chars");
    }
}
