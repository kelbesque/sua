use std::fmt;

use actix_web::{Path, Json};
use actix_web::http::StatusCode;
use chrono::DateTime;
use chrono::offset;
use diesel::pg::PgConnection;
use diesel::prelude::*;

use crate::accounts;
use crate::db;
use crate::models;
use crate::*;
use crate::schema::posts::dsl as post_dsl;
use crate::schema::accounts::dsl as account_dsl;

#[derive(Serialize)]
pub struct Post {
    pub id: i32,
    pub from: String,
    pub to: Vec<String>,
    pub privacy: i32,
    pub content_warning: Option<String>,
    pub text: Option<String>,
    pub image_data: Option<serde_json::Value>,
    pub time: chrono::NaiveDateTime
}

impl Post {
    fn new(post: models::Post,
           from: models::Account,
           to: Vec<models::Account>) -> Post {
        Post {
            id: post.id,
            from: {
                let d = accounts::decompose_url(&from.url).unwrap();
                format!("{}@{}", d.1, d.0)
            },
            to: to.iter().map(|a| {
                    let d = accounts::decompose_url(&a.url).unwrap();
                    format!("{}@{}", d.1, d.0)
                }).collect::<Vec<String>>(),
            privacy: post.privacy,
            content_warning: post.content_warning,
            text: post.text,
            image_data: post.image_data,
            time: post.time
        }
    }
}

fn get_public_posts(connection: &PgConnection, id: i32) -> Vec<models::Post> {
    post_dsl::posts.filter(post_dsl::privacy.eq(0))
        .filter(post_dsl::src.eq(id))
        .order(post_dsl::time.desc())
        .load::<models::Post>(connection)
        .expect("Error loading posts")
}

pub fn get_user_timeline(info: Path<(i32,)>) -> Json<Vec<Post>> {
    let connection = db::db_connect();

    let results = get_public_posts(&connection, info.0);

    let mut posts = Vec::new();

    for post in results {
        let now = offset::Local::now();
        let tz = now.offset();
        let post_time = DateTime::<offset::Local>::from_utc(post.time, *tz);

        let from = account_dsl::accounts.find(post.src).first::<models::Account>(&connection)
            .expect("Error loading `from` account");
        let to = accounts::get_dests_for_post(&post, Some(&connection));

        posts.push(Post::new(post, from, to));
    }

    Json(posts)
}
