use crate::schema::*;
use chrono::NaiveDateTime;
use serde::Serialize;

#[derive(Queryable, Serialize)]
pub struct Comment {
    pub id: i32,
    pub body: String,
    pub create_at: NaiveDateTime,
    pub post: i32,
    pub author: i32,
}
#[derive(Insertable)]
#[table_name = "comments"]
pub struct NewComment<'a> {
    pub body: &'a str,
    pub post: &'a i32,
    pub author: &'a i32,
}
#[derive(Queryable, Serialize)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
    pub author: i32,
    pub create_at: NaiveDateTime,
}

#[derive(Queryable, Serialize)]
pub struct PostAndUser {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub author: i32,
    pub create_at: NaiveDateTime,
    pub username: String,
}

#[derive(Insertable)]
#[table_name = "posts"]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub body: &'a str,
    pub published: &'a bool,
    pub author: &'a i32,
}

#[derive(Debug, Queryable, Serialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
}
#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub password: &'a str,
}
