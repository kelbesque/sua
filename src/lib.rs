use std::fmt;

extern crate actix_web;
use actix_web::{Path, Responder};

extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

use serde_json::{Error};

use serde::ser::{Serialize, Serializer};

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
