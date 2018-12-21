use std::fmt;

extern crate actix_web;
use actix_web::{Path, Responder};

struct Version(u32, u32, u32);

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\"{}.{}.{}\"", self.0, self.1, self.2)
    }
}

const NAME: &str = "sua-server";
const VERSION: Version = Version(0,0,1);

pub fn index(info: Path<(u32, String)>) -> impl Responder {
    format!("Hello {}! id:{}", info.1, info.0)
}

pub fn root(info: Path<()>) -> impl Responder {
    format!("Hello, world!")
}

pub fn health(info: Path<()>) -> impl Responder {
    format!("{{{:?}:{:?},{:?}:{},{:?}:{:?}}}",
            "name", NAME,
            "version", VERSION,
            "status", "OK"
            )
}
