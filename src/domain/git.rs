use super::commit::Commit;

#[derive(Debug)]
pub struct Git {
    last_commit_id: u32,
    name: String,
}

impl Git {
    pub fn new(id: Option<u32>, name: String) -> Self {
        let last_commit_id = match id {
            None => 0,
            Some(v) => v,
        };
        Git {
            last_commit_id,
            name,
        }
    }

    pub fn commit(&mut self, message: String) -> Commit {
        self.last_commit_id += 1;
        Commit::new(self.last_commit_id, message)
    }

    pub fn echo(&self) {
        println!("{}, {}", self.last_commit_id, self.name)
    }
}
