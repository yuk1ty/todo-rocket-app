use std::cell::Cell;
use std::borrow::BorrowMut;

#[derive(Clone)]
pub struct Id {
    id: Cell<u64>,
}

impl Id {
    pub fn new() -> Id {
        Id { id: Cell::new(0) }
    }

    pub fn incr(&mut self) {
        let new_id = self.clone().id.into_inner() + 1;
        self.id.borrow_mut().set(new_id);
    }

    pub fn unwrap(&self) -> u64 {
        self.clone().id.into_inner()
    }
}
