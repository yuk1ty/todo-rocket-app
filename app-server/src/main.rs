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
use application::endpoints::*;

pub mod application;
pub mod domain;
pub mod infra;

pub fn main() {
    rocket::ignite()
        .mount("/", routes![hc])
        .mount(
            "/tasks",
            routes![
                tasks::preflight_tasks_new,
                tasks::preflight_tasks_update,
                tasks::list,
                tasks::new,
                tasks::update,
            ],
        )
        .manage(Mutex::new(HashMap::<u64, Task>::new()))
        .manage(Mutex::new(Id::new()))
        .launch();
}
