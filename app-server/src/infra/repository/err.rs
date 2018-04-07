pub struct RepositoryErr {
    pub message: String,
}

impl RepositoryErr {
    pub fn new(message: String) -> RepositoryErr {
        RepositoryErr { message }
    }
}
