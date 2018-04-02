extern crate rocket;

use domain::model::task::*;
use rocket_contrib::Json;
use rocket::State;
use std::collections::HashMap;
use std::sync::Mutex;
use infra::util::*;

type TaskRepository = Mutex<HashMap<u64, Task>>;
type IdGenerator = Mutex<Id>;

#[get("/hc")]
fn hc() -> &'static str {
    "OK"
}

#[get("/tasks/list")]
fn list(repository: State<TaskRepository>) -> Json<Vec<Task>> {
    let mut list = Vec::<Task>::new();
    for (_, task) in repository.lock().unwrap().iter() {
        list.push(task.clone());
    }
    Json(list)
}

#[post("/tasks/new", format = "application/json", data = "<task>")]
fn new(task: Json<Task>, _repository: State<TaskRepository>, gen: State<IdGenerator>) -> Json<TaskResponse> {
    let mut mut_repository = _repository.lock().expect("Repository is locked.");
    let copied_task: Task = task.into_inner().clone();
    let id = gen.lock().expect("Id generator is locked").incr();

    mut_repository.insert(id.unwrap(), copied_task.clone());

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
