#![feature(plugin)]
#![plugin(rocket_codegen)]
extern crate rocket;
extern crate rocket_contrib;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

pub mod endpoints;
pub mod entity;

pub fn main() {
    rocket::ignite()
        .mount("/", routes![endpoints::hc, endpoints::list, endpoints::new])
        .launch();
}
