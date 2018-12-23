use std::fmt;

use actix_web::{Path, HttpResponse};
use actix_web::http::StatusCode;
use chrono::DateTime;
use chrono::offset;
use diesel::pg::PgConnection;
use diesel::prelude::*;

use crate::db;
use crate::models::*;
use crate::*;
use crate::schema::posts::dsl as post_dsl;
use crate::schema::accounts::dsl as account_dsl;
    
fn get_public_posts(connection: &PgConnection, id: i32) -> Vec<Post> {
    post_dsl::posts.filter(post_dsl::privacy.eq(0))
        .filter(post_dsl::src.eq(id))
        .order(post_dsl::time.desc())
        .load::<Post>(connection)
        .expect("Error loading posts")
}

pub fn get_user_timeline(info: Path<(i32,)>) -> HttpResponse {
    let connection = db::db_connect();

    let results = get_public_posts(&connection, info.0);

    let mut response = String::new();

    for post in results {
        let now = offset::Local::now();
        let tz = now.offset();
        let post_time = DateTime::<offset::Local>::from_utc(post.time, *tz);

        let from = account_dsl::accounts.find(post.src).first::<Account>(&connection)
            .expect("Error loading `from` account");
        let to = account_dsl::accounts.find(post.dest).first::<Account>(&connection)
            .expect("Error loading `to` account");

        response = format!("{}<p>from: {}<br />to: {}<br />at: {}<br /><p>{}</p></p>", 
                response,
                from.url, 
                to.url, 
                post_time, 
                post.text.unwrap());
    }

    response = format!("<html>{}</html>",response);

    HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(response)
}
