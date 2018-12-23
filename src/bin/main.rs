extern crate actix_web;
extern crate getopts;

use std::env;
use std::process;

use actix_web::{http, server, App};
use getopts::Options;

use sua;

struct Config {
    port: u16,
}

impl Config {
    fn new(port: u16) -> Config {
        Config{port}
    }
}

fn parse_options() -> Config {
    let args: Vec<String> = env::args().collect();

    let mut opts = Options::new();
    opts.reqopt("p", "port", "set port to start server on", "SUA_PORT");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => {
            eprintln!("{}",f.to_string());
            process::exit(1);
        }
    };

    Config::new(
        matches.opt_str("p")
            .unwrap()
            .parse()
            .ok()
            .unwrap()
        )
}

fn main() {
    let config = parse_options();
    let bind_address = format!("127.0.0.1:{}", config.port);

    server::new(
        || App::new()
            .route("/", http::Method::GET, sua::root)
            .route("/health", http::Method::GET, sua::health)
            .route("/{id}", http::Method::GET, sua::posts::get_user_timeline)
        )
        .bind(bind_address).unwrap()
        .run();
}
