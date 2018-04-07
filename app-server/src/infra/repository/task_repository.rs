use super::err::*;
use domain::model::task::*;
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

pub trait Repository<T> {
    fn into_list(self) -> Vec<T>;

    fn store(&self, id: u64, v: T) -> Option<T>;

    fn update(&self, v: T) -> Result<T, RepositoryErr>;
}

pub struct TaskRepository {
    repository: Rc<RefCell<HashMap<u64, Task>>>,
}

impl Repository<Task> for TaskRepository {
    fn into_list(self) -> Vec<Task> {
        let mut list = Vec::<Task>::new();

        for (_, task) in self.repository.borrow_mut().iter() {
            list.push(task.clone());
        }

        list
    }

    fn store(&self, id: u64, v: Task) -> Option<Task> {
        self.repository.borrow_mut().insert(id, v)
    }

    fn update(&self, v: Task) -> Result<Task, RepositoryErr> {
        let mut repository = self.repository.borrow_mut();

        if repository.contains_key(&v.id.unwrap()) {
            let update_task = Task::copy(v.id, v);
            let saved = repository.insert(update_task.id.unwrap(), update_task);
            match saved {
                Some(task) => Ok(task),
                None => Err(RepositoryErr::new("Can't update task".to_string()))
            }
        } else {
            Err(RepositoryErr::new("Not Found Task".to_string()))
        }
    }
}
