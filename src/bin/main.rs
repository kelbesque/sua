extern crate actix_web;
use actix_web::{http, server, App};

use sua;

fn main() {
    server::new(
        || App::new()
            .route("/", http::Method::GET, sua::root)
            .route("/health", http::Method::GET, sua::health)
        )
        .bind("127.0.0.1:8080").unwrap()
        .run();
}
