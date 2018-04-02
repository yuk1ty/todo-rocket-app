pub struct Id {
    id: u64,
}

impl Id {
    pub fn new() -> Id {
        Id { id: 0 }
    }

    pub fn incr(&self) -> Id {
        let new_id = self.id + 1;
        Id { id: new_id }
    }

    pub fn unwrap(&self) -> u64 {
        self.id
    }
}
