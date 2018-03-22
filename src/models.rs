use super::schema::posts;
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
