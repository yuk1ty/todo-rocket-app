#![feature(plugin)]
#![plugin(rocket_codegen)]
extern crate rocket;
extern crate rocket_contrib;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

use std::sync::Mutex;
use std::collections::HashMap;
use domain::model::task::*;

pub mod application;
pub mod domain;

pub fn main() {
    rocket::ignite()
        .mount(
            "/",
            routes![
                application::endpoints::hc,
                application::endpoints::list,
                application::endpoints::new
            ],
        )
        .manage(Mutex::new(HashMap::<u64, Task>::new()))
        .launch();
}
