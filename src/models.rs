use schema::posts;
use pulldown_cmark::{html, Parser};
use serde::ser::{Serialize, Serializer};

pub struct MarkdownText(String);

impl Serialize for MarkdownText {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        let parser = Parser::new(&self.0.as_str());
        let mut html_buf = String::new();
        html::push_html(&mut html_buf, parser);
        println!("formated : {}", html_buf);
        serializer.serialize_str(html_buf.as_str())
    }
}

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
