use std::fmt;

extern crate actix_web;
use actix_web::{Path, Responder};

const NAME: &str = "sua-server";
const VERSION: Version = Version(0,0,1);

struct Version(u32, u32, u32);

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\"{}.{}.{}\"", self.0, self.1, self.2)
    }
}

pub fn root(_info: Path<()>) -> impl Responder {
    format!("Hello, world!")
}

pub fn health(_info: Path<()>) -> impl Responder {
    format!("{{{:?}:{:?},{:?}:{},{:?}:{:?}}}",
            "name", NAME,
            "version", VERSION,
            "status", "OK"
            )
}
