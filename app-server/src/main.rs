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
        .mount("/", routes![application::endpoints::hc,])
        .mount(
            "/tasks",
            routes![
                application::endpoints::tasks::preflight_tasks_new,
                application::endpoints::tasks::preflight_tasks_update,
                application::endpoints::tasks::list,
                application::endpoints::tasks::new,
                application::endpoints::tasks::update,
            ],
        )
        .manage(Mutex::new(HashMap::<u64, Task>::new()))
        .manage(Mutex::new(Id::new()))
        .launch();
}
