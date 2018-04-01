#![feature(plugin)]
#![plugin(rocket_codegen)]
extern crate rocket;
extern crate rocket_contrib;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

pub mod application;
pub mod domain;

pub fn main() {
    rocket::ignite()
        .mount("/", routes![application::endpoints::hc, application::endpoints::list, application::endpoints::new])
        .launch();
}
