use schema::posts;
use pulldown_cmark::{html, Parser};

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

impl NewPost {
    pub fn validate(&self) -> Result<(), &'static str> {
        if self.title.is_empty() {
            return Err("Title shall not be emty");
        } else if self.body.is_empty() {
            return Err("Body shall not be emty");
        }
        Ok(())
    }
}

impl Post {
    pub fn format(self) -> Post {
        let to_format = self.body.to_owned();
        let parser = Parser::new(&to_format.as_str());
        let mut formated_post = self;
        formated_post.body.truncate(0);
        html::push_html(&mut formated_post.body, parser);
        formated_post
    }
}
