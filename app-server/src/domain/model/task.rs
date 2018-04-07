use std::io::Cursor;

use rocket::request::Request;
use rocket::response::{self, Responder, Response};
use serde_json;

#[derive(Serialize, Deserialize, Clone)]
pub struct Task {
    pub id: Option<u64>,
    pub name: String,
    pub due: String,
    pub done: bool,
}

impl Task {
    pub fn new(id: Option<u64>, name: String, due: String, done: bool) -> Task {
        Task {
            id,
            name,
            due,
            done,
        }
    }

    pub fn copy(id: Option<u64>, source: Task) -> Task {
        Task::new(id, source.name, source.due, source.done)
    }
}

impl<'r> Responder<'r> for Task {
    fn respond_to(self, _: &Request) -> response::Result<'r> {
        Response::build()
            .raw_header("Content-Type", "application/json")
            .raw_header("Access-Control-Allow-Origin", "*")
            .raw_header(
                "Access-Control-Allow-Methods",
                "GET,POST,PATCH,HEAD,OPTIONS",
            )
            .raw_header("Access-Control-Allow-Headers", "Content-Type")
            .raw_header("Access-Control-Allow-Credentials", "true")
            .sized_body(Cursor::new(format!(
                "{}",
                serde_json::to_string(&self).unwrap()
            )))
            .ok()
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TaskList {
    pub tasks: Vec<Task>,
}

impl TaskList {
    pub fn new(tasks: Vec<Task>) -> TaskList {
        TaskList { tasks }
    }
}

impl<'r> Responder<'r> for TaskList {
    fn respond_to(self, _: &Request) -> response::Result<'r> {
        Response::build()
            .raw_header("Content-Type", "application/json")
            .raw_header("Access-Control-Allow-Origin", "*")
            .raw_header(
                "Access-Control-Allow-Methods",
                "GET",
            )
            .raw_header("Access-Control-Allow-Headers", "Content-Type")
            .raw_header("Access-Control-Allow-Credentials", "true")
            .sized_body(Cursor::new(format!(
                "{}",
                serde_json::to_string(&self.tasks).unwrap()
            )))
            .ok()
    }
}
