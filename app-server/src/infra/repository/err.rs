pub struct RepositoryErr {
    pub message: String,
}

impl RepositoryErr {
    fn new(message: String) -> RepositoryErr {
        RepositoryErr { message }
    }
}
