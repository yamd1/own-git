use super::commit::Commit;

pub struct Git {
    name: String,
}

impl Git {
    pub fn new(name: String) -> Self {
        Git { name }
    }

    pub fn commit(&self, message: String) -> Commit {
        Commit::new(message)
    }

    pub fn echo(&self) -> &str {
        &self.name
    }
}
