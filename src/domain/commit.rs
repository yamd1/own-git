#[derive(Debug, Clone)]
pub struct Commit {
    id: u32,
    message: String,
}

impl Commit {
    pub fn new(id: u32, message: String) -> Self {
        Commit { id, message }
    }

    pub fn echo_message(&self) {
        println!("{}, {}", self.id, self.message);
    }
}
