use domain::model::task::*;
use domain::model::preflight::*;
use rocket_contrib::Json;
use rocket::State;
use rocket::response::status::NotFound;
use std::collections::HashMap;
use std::sync::Mutex;
use infra::util::*;

type TaskRepository = Mutex<HashMap<u64, Task>>;
type IdGenerator = Mutex<Id>;

#[route(OPTIONS, "/new")]
fn preflight_tasks_new() -> PreflightResponse {
    PreflightResponse()
}

#[route(OPTIONS, "/update")]
fn preflight_tasks_update() -> PreflightResponse {
    PreflightResponse()
}

#[get("/list")]
fn list(repository: State<TaskRepository>) -> TaskList {
    let mut list = Vec::<Task>::new();
    for (_, task) in repository.lock().unwrap().iter() {
        list.push(task.clone());
    }
    TaskList::new(list)
}

#[post("/new", format = "application/json", data = "<task>")]
fn new(task: Json<Task>, _repository: State<TaskRepository>, gen: State<IdGenerator>) -> Task {
    let mut mut_repository = _repository.lock().expect("Repository is locked.");

    let id;
    {
        let mut generator = gen.lock().expect("Id generator is locked.");
        generator.incr();
        id = generator.unwrap();
    }

    let copied_task: Task = task.into_inner();
    let save_task = Task::copy(Some(id), copied_task);

    mut_repository.insert(id, save_task.clone());

    save_task
}

#[patch("/update", format = "application/json", data = "<task>")]
fn update(task: Json<Task>, _repository: State<TaskRepository>) -> Result<Task, NotFound<String>> {
    let mut mut_repository = _repository.lock().expect("Repository is locked.");
    let copied_task: Task = task.into_inner();
    let maybe_task_id: Option<u64> = copied_task.id;

    match maybe_task_id {
        Some(task_id) => if mut_repository.contains_key(&task_id) {
            let update_task = Task::copy(Some(task_id), copied_task);

            let saved = mut_repository
                .insert(task_id, update_task.clone())
                .expect(&format!("Can't insert task: task_id #{}", update_task.id.unwrap()));
            Ok(saved)
        } else {
            Err(NotFound(format!("Task Not Found: {}", &copied_task.name)))
        },
        None => Err(NotFound(format!("Task Not Found: {}", &copied_task.name))),
    }
}
