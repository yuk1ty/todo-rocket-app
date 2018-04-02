use super::err::*;
use domain::model::task::*;
use std::sync::Mutex;
use std::collections::HashMap;

pub trait Repository<T> {
    fn into_list() -> Vec<T>;

    fn store(v: T) -> T;

    fn update(v: T) -> Result<T, RepositoryErr>;
}

pub struct TaskRepository {
    repository: Mutex<HashMap<u64, Task>>,
}
