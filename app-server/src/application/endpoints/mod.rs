extern crate rocket;

//use entity::task::*;
use domain::model::task::*;
use rocket_contrib::Json;

use rocket::State;
use std::collections::HashMap;
use std::sync::Mutex;

type TaskRepository = Mutex<HashMap<u64, Task>>;

#[get("/hc")]
fn hc() -> &'static str {
    "OK"
}

#[get("/tasks/list")]
fn list(repository: State<TaskRepository>) -> Json<Vec<Task>> {
    Json(vec![
        Task::new(
            Some(1),
            "taskname".to_string(),
            "20180501".to_string(),
            false,
        ),
        Task::new(
            Some(2),
            "tasknameB".to_string(),
            "20180501".to_string(),
            false,
        ),
    ])
}

#[post("/tasks/new", format = "application/json", data = "<task>")]
fn new(task: Json<Task>, _repository: State<TaskRepository>) -> Json<TaskResponse> {
    let mut mut_repository = _repository.lock().expect("Repository is locked.");
    let copied_task: Task = task.into_inner().clone();

    // TODO ID generation
    mut_repository.insert(1, copied_task.clone());
    Json(TaskResponse::new("OK".to_string(), None, Some(copied_task)))
}

#[put("/tasks/update", format = "application/json", data = "<task>")]
fn update(task: Json<Task>, _repository: State<TaskRepository>) -> Json<TaskResponse> {
    let mut mut_repository = _repository.lock().expect("Repository is locked.");
    let copied_task: Task = task.into_inner().clone();
    let maybe_task_id: Option<u64> = copied_task.id;

    match maybe_task_id {
        Some(task_id) => if mut_repository.contains_key(&task_id) {
            mut_repository.insert(task_id, copied_task.clone());
            Json(TaskResponse::new("OK".to_string(), None, Some(copied_task)))
        } else {
            Json(TaskResponse::new(
                "503".to_string(),
                Some("Task Not Found".to_string()),
                None,
            ))
        },
        None => Json(TaskResponse::new(
            "503".to_string(),
            Some("Task Not Found".to_string()),
            None,
        )),
    }
}
