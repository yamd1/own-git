pub struct Commit {
    id: u32,
    message: String,
}

impl Commit {
    pub fn new(message: String) -> Self {
        Commit { id: 1, message }
    }

    pub fn echo_message(&self) -> &str {
        &self.message
    }
}
