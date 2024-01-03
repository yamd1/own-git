use super::commit::Commit;

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

    pub fn commit(&self, message: String) -> Commit {
        Commit::new(message)
    }

    pub fn echo(&self) {
        println!("{}, {}", &self.last_commit_id, &self.name)
    }
}
