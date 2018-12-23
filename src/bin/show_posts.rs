extern crate chrono;
extern crate diesel;
extern crate sua;

use chrono::DateTime;
use chrono::offset;

use self::sua::*;
use self::models::*;
use self::diesel::prelude::*;

fn main() {
    use sua::schema::posts::dsl::*;
    use sua::schema::accounts::dsl::*;

    let connection = sua::db::db_connect();

    let results = posts.filter(privacy.eq(0))
        .order(time.desc())
        .load::<Post>(&connection)
        .expect("Error loading posts");

    println!("Display {} posts", results.len());

    for post in results {
        let now = offset::Local::now();
        let tz = now.offset();
        let post_time = DateTime::<offset::Local>::from_utc(post.time, *tz);

        let from = accounts.find(post.src).first::<Account>(&connection)
            .expect("Error loading `from` account");
        let to = accounts.find(post.dest).first::<Account>(&connection)
            .expect("Error loading `from` account");

        println!("from: {}", from.url);
        println!("to: {}", to.url);
        println!("at: {}", post_time);
        println!("text: {}", post.text.unwrap());
    }
}
