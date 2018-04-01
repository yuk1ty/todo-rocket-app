extern crate rocket;

use entity::task::*;
use rocket_contrib::Json;

#[get("/hc")]
fn hc() -> &'static str {
    "OK"
}

#[get("/tasks/list")]
fn list() -> Json<Vec<Task>> {
    Json(vec![
        Task::new(1, "taskname".to_string(), "20180501".to_string(), false),
        Task::new(2, "tasknameB".to_string(), "20180501".to_string(), false),
    ])
}

#[post("/tasks/new", format = "application/json", data = "<task>")]
fn new(task: Json<Task>) -> Json<TaskResponse> {
    Json(TaskResponse::new(
        "202 Accepted".to_string(),
        None,
        Some(task.into_inner()),
    ))
}

#[put("/tasks/update", format = "application/json", data = "<task>")]
fn update(task: Json<Task>) -> Json<TaskResponse> {
    Json(TaskResponse::new(
        "202 Accepted".to_string(),
        None,
        Some(task.into_inner()),
    ))
}
