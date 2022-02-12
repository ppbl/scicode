use std::time::SystemTime;

use serde::Serialize;

use crate::schema::{comments, posts};

#[derive(Queryable, Serialize)]
pub struct Comment {
    pub id: i32,
    pub body: String,
    pub create_at: SystemTime,
    pub post_id: Option<i32>,
}
#[derive(Insertable)]
#[table_name = "comments"]
pub struct NewComment<'a> {
    pub body: &'a str,
    pub post_id: &'a i32,
}

#[derive(Queryable, Serialize)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
    pub create_at: SystemTime,
}
#[derive(Insertable)]
#[table_name = "posts"]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub body: &'a str,
    pub published: &'a bool,
}
