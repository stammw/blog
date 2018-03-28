use schema::posts;

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
