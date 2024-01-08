#[derive(Debug, Clone)]
pub struct Commit {
    id: u32,
    pub parent: Box<Option<Commit>>,
    message: String,
}

impl Commit {
    pub fn new(id: u32, parent: Box<Option<Commit>>, message: String) -> Self {
        Commit {
            id,
            parent,
            message,
        }
    }

    pub fn echo_message(&self) {
        println!("{}, {}", self.id, self.message);
    }
}
