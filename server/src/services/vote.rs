use crate::{
    auth::get_claims,
    db,
    models::{NewVote, Post, PostThumbs},
};
use actix_web::{http::header::AUTHORIZATION, post, web, HttpRequest, HttpResponse, Responder};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct Body {
    post: i32,
    vote_type: String,
}

#[derive(Serialize)]
struct ResBody {
    ups: i32,
    downs: i32,
    voting: Option<bool>,
}

#[post("/vote")]
async fn vote(req_body: web::Json<Body>, req: HttpRequest) -> impl Responder {
    use crate::schema::posts;
    use crate::schema::posts_thumbs::dsl::*;
    let token = req.headers().get(AUTHORIZATION);
    if let Some(token) = token {
        let claims = get_claims(token.to_str().unwrap());
        let next_voting = if req_body.vote_type == "up" {
            Some(true)
        } else if req_body.vote_type == "down" {
            Some(false)
        } else {
            None::<bool>
        };

        let conn = db::get_connection();
        let prev_vote = posts_thumbs
            .filter(post.eq(req_body.post))
            .filter(author.eq(claims.userid))
            .get_result::<PostThumbs>(&conn);
        if let Ok(prev_vote) = prev_vote {
            if prev_vote.voting == next_voting {
                return HttpResponse::Ok().body("No action required");
            }
            diesel::update(posts_thumbs.filter(author.eq_all(claims.userid)))
                .set(voting.eq(next_voting))
                .load::<PostThumbs>(&conn)
                .expect("Failed to voting");
        } else {
            let vote = NewVote {
                post: &req_body.post,
                author: &claims.userid,
                voting: &next_voting.unwrap(),
            };
            diesel::insert_into(posts_thumbs)
                .values(&vote)
                .load::<PostThumbs>(&conn)
                .expect("Failed to voting");
        }

        let res = posts::table
            .find(req_body.post)
            .get_result::<Post>(&conn)
            .expect("Failed to get votes");
        // Calculate the number of votes
        let mut ups = res.ups;
        let mut downs = res.downs;
        if let Some(next_voting) = next_voting {
            if next_voting {
                match prev_vote {
                    Ok(prev_vote) => match prev_vote.voting {
                        Some(false) => {
                            ups = res.ups + 1;
                            downs = res.downs - 1;
                        }
                        _ => {
                            ups = res.ups + 1;
                        }
                    },
                    Err(_) => {
                        ups = res.ups + 1;
                    }
                }
            } else {
                match prev_vote {
                    Ok(prev_vote) => match prev_vote.voting {
                        Some(true) => {
                            ups = res.ups - 1;
                            downs = res.downs + 1;
                        }
                        _ => {
                            downs = res.downs + 1;
                        }
                    },
                    Err(_) => {
                        downs = res.downs + 1;
                    }
                }
            }
        } else {
            if prev_vote.unwrap().voting.unwrap() == true {
                ups = res.ups - 1;
            } else {
                downs = res.downs - 1;
            }
        }
        diesel::update(posts::table.find(req_body.post))
            .set((posts::ups.eq(ups), posts::downs.eq(downs)))
            .execute(&conn)
            .expect("Failed to voting");
        HttpResponse::Ok().json(ResBody {
            ups,
            downs,
            voting: next_voting,
        })
    } else {
        HttpResponse::Ok().body("please sgin in")
    }
}
