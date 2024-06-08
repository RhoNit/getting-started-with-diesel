use super::schema::posts;

#[derive(Queryable)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub is_published: bool,
}

#[derive(Insertable)]
#[diesel(table_name = posts)]
pub struct NewPost {
    pub title: String,
    pub body: String,
}

impl NewPost {
    pub fn new(title: String, body: String) -> NewPost {
        NewPost {
            title,
            body
        }
    }
}