use crate::schema::*;
use chrono::NaiveDateTime;
use diesel::Insertable;
use serde::Serialize;

#[derive(Queryable, Serialize)]
pub struct CommentAndUser {
    pub id: i32,
    pub body: String,
    pub create_at: NaiveDateTime,
    pub author: SomeUser,
}
#[derive(Queryable, Serialize)]
pub struct Comment {
    pub id: i32,
    pub body: String,
    pub create_at: NaiveDateTime,
    pub post: Option<i32>,
    pub author: i32,
}
#[derive(Insertable)]
#[table_name = "comments"]
pub struct NewComment<'a> {
    pub body: &'a str,
    pub post: &'a i32,
    pub author: &'a i32,
}

#[derive(Insertable)]
#[table_name = "posts_thumbs"]
pub struct NewVote<'a> {
    pub post: &'a i32,
    pub author: &'a i32,
    pub voting: &'a bool,
}

#[derive(Insertable)]
#[table_name = "topics"]
pub struct NewTopic<'a> {
    pub name: &'a str,
}

#[derive(Debug, Queryable, Serialize)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
    pub author: i32,
    pub topics: Vec<i32>,
    pub create_at: NaiveDateTime,
    pub ups: i32,
    pub downs: i32,
}
#[derive(Queryable, Clone, Copy, Serialize)]
pub struct PostThumbs {
    pub id: i32,
    pub post: i32,
    pub author: i32,
    pub voting: Option<bool>,
    pub create_at: NaiveDateTime,
}
#[derive(Debug, Clone, Queryable, Serialize)]
pub struct SomeUser {
    pub id: i32,
    pub username: String,
    pub avatar_url: Option<String>,
}
#[derive(Queryable, Serialize)]
pub struct PostAndUser {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub topics: Vec<i32>,
    pub author: SomeUser,
    pub create_at: NaiveDateTime,
    pub ups: i32,
    pub downs: i32,
}
#[derive(Queryable, Serialize)]
pub struct PostAndUserAndTopics {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub topics: Vec<Topics>,
    pub author: SomeUser,
    pub create_at: NaiveDateTime,
    pub ups: i32,
    pub downs: i32,
    pub voting: Option<bool>,
}

#[derive(Insertable)]
#[table_name = "posts"]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub body: &'a str,
    pub topics: &'a Vec<i32>,
    pub published: &'a bool,
    pub author: &'a i32,
}

#[derive(Debug, Clone, Queryable, Serialize)]
pub struct Topics {
    pub id: i32,
    pub name: String,
}
#[derive(Debug, Queryable, Serialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: Option<String>,
    pub github_id: Option<i64>,
    pub github_url: Option<String>,
    pub avatar_url: Option<String>,
}
#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub password: Option<&'a str>,
    pub github_id: Option<&'a i64>,
    pub github_url: Option<&'a str>,
    pub avatar_url: Option<&'a str>,
}
