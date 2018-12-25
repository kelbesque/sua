extern crate actix_web;
extern crate chrono;

#[macro_use]
extern crate diesel;

extern crate dotenv;

extern crate regex;

extern crate serde;

#[macro_use]
extern crate serde_derive;

extern crate serde_json;

use std::fmt;

use actix_web::{Path, Responder};
use serde::ser::{Serialize, Serializer};
use serde_json::{Error};

pub mod accounts;
pub mod db;
pub mod models;
pub mod schema;
pub mod posts;

const HEALTH_OK: Health = Health{
    name: "sua-server",
    version: Version(0,0,1),
    status: "OK",
};

#[derive(Deserialize)]
struct Version(u32, u32, u32);

#[derive(Serialize, Deserialize)]
struct Health {
    name: &'static str,
    version: Version,
    status: &'static str,
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}.{}.{}",
               self.0, self.1, self.2)
    }
}

impl Serialize for Version {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            serializer.serialize_str(&format!("{}", self))
        }
}

pub fn root(_info: Path<()>) -> impl Responder {
    format!("Hello, world!")
}

pub fn health(_info: Path<()>) -> impl Responder {
    let j = serde_json::to_string(&HEALTH_OK).unwrap();
    format!("{}", j)
}
