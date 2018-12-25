use diesel::pg::PgConnection;
use diesel::prelude::*;

use regex::Regex;

use crate::db;
use crate::models::{Account, Post, PostDest};
use crate::*;

pub fn decompose_url(url: &String) -> Option<(String, String)> {
    let re = Regex::new(r"^https?://([^/]+)/?.*/([^/]+)$").unwrap();

    match re.captures(url) {
        Some(m) => Some(
                (String::from(m.get(1).unwrap().as_str()),
                String::from(m.get(2).unwrap().as_str()))
                ),
        None => None
    }
}

pub fn get_dests_for_post(post: &Post, connection: Option<&PgConnection>) -> Vec<models::Account> {
    use crate::schema::accounts::dsl::{accounts, id, url};
    use crate::schema::posts_dests::dsl::{posts_dests, post_id};

    let tmp: PgConnection;

    let connection = match connection {
        Some(connection) => connection,
        None => {
            tmp = db::db_connect();
            &tmp
        }
    };

    let dests = posts_dests.filter(post_id.eq(post.id))
                          .load::<PostDest>(connection)
                          .expect("Error loading post targets")
                          .iter()
                          .map(|d| { d.dest_id })
                          .collect::<Vec<_>>();
                

    accounts.filter(
            id.eq_any(dests)
        ).load::<Account>(connection).expect("Error loading associated account for post targets")
}
