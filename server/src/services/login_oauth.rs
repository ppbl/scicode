use crate::{
    auth::{generate_token, Claims},
    db,
    models::{NewUser, User},
};
use actix_web::{get, web, HttpResponse, Responder};
use diesel::prelude::*;
use reqwest::header::{HeaderValue, ACCEPT, AUTHORIZATION, USER_AGENT};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{
    collections::HashMap,
    env,
    time::{SystemTime, UNIX_EPOCH},
    usize,
};

#[derive(Deserialize)]
struct AuthQuery {
    code: String,
}

#[derive(Debug, Deserialize)]
struct AuthBody {
    access_token: String,
    scope: String,
    token_type: String,
}

#[derive(Serialize)]
struct Response<T> {
    status: &'static str,
    data: T,
}

fn get_after_days(n: u64) -> usize {
    (SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
        + n * 24 * 3600 * 1000) as usize
}
#[get("/login/oauth")]
async fn login_oauth(query: web::Query<AuthQuery>) -> impl Responder {
    use crate::schema::users::dsl::*;
    let mut map = HashMap::new();
    map.insert("code", query.code.clone());
    map.insert("client_id", env::var("CLIENT_ID").unwrap());
    map.insert("client_secret", env::var("CLIENT_SECRET").unwrap());
    map.insert("redirect_uri", env::var("REDIRECT_URI").unwrap());
    let client = reqwest::Client::new();
    let body = client
        .post("https://github.com/login/oauth/access_token")
        .form(&map)
        .header(ACCEPT, HeaderValue::from_str("application/json").unwrap())
        .send()
        .await
        .expect("Failed to request")
        .json::<AuthBody>()
        .await
        .expect("Failed to parse");
    let user_info = client
        .get("https://api.github.com/user")
        .header(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("token {}", body.access_token)).unwrap(),
        )
        .header(USER_AGENT, HeaderValue::from_str("Scicode").unwrap())
        .send()
        .await
        .expect("Failed to request")
        .text()
        .await
        .expect("Failed to parse");
    let conn = db::get_connection();
    let user_info: Value = serde_json::from_str(&user_info).unwrap();
    let githubid = user_info["id"].as_i64().unwrap();
    let user_r = users
        .filter(github_id.eq(githubid))
        .get_result::<User>(&conn);
    let user: User;
    match user_r {
        Ok(exist_user) => user = exist_user,
        Err(_) => {
            let new_user = NewUser {
                username: user_info["login"].as_str().unwrap(),
                github_id: Some(&githubid),
                github_url: user_info["html_url"].as_str(),
                avatar_url: user_info["avatar_url"].as_str(),
                password: None,
            };
            user = diesel::insert_into(users)
                .values(&new_user)
                .get_result::<User>(&conn)
                .expect("Error saving new user");
        }
    }
    let claims = Claims {
        userid: user.id,
        username: user.username,
        exp: get_after_days(7),
    };
    let token = generate_token(claims);
    HttpResponse::Ok().body(format!(
        "<script>
            window.opener.localStorage.setItem('token', '{}');
            window.opener.localStorage.setItem('userid', '{}');
            window.opener.location.reload();
            window.close();
        </script>",
        token, user.id
    ))
}
