#![feature(plugin)]
#![plugin(rocket_codegen)]
extern crate rocket;
extern crate rocket_contrib;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

use std::sync::Mutex;
use std::collections::HashMap;
use domain::model::task::*;
use infra::util::Id;

pub mod application;
pub mod domain;
pub mod infra;

pub fn main() {
    rocket::ignite()
        .mount(
            "/",
            routes![
                application::endpoints::hc,
                application::endpoints::preflight_tasks_new,
                application::endpoints::preflight_tasks_update,
                application::endpoints::list,
                application::endpoints::new,
                application::endpoints::update
            ],
        )
        .manage(Mutex::new(HashMap::<u64, Task>::new()))
        .manage(Mutex::new(Id::new()))
        .launch();
}
