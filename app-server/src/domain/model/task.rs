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
}

#[derive(Serialize, Deserialize)]
pub struct TaskResponse {
    status: String,
    reason: Option<String>,
    task: Option<Task>
}

impl TaskResponse {
    pub fn new(status: String, reason: Option<String>, task: Option<Task>) -> TaskResponse {
        TaskResponse {
            status,
            reason,
            task
        }
    }
}