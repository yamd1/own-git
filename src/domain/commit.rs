#[derive(Debug, Clone)]
pub struct Commit {
    pub id: u32,
    pub parent: Option<Box<Commit>>,
    message: String,
}

impl Commit {
    pub fn new(id: u32, parent: Option<Commit>, message: String) -> Self {
        let parent = parent.map(|p| Box::new(p));
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
