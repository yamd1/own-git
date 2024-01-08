use std::borrow::{Borrow, BorrowMut};

use super::commit::Commit;

#[derive(Debug, Clone)]
pub struct Git {
    last_commit_id: u32,
    name: String,
    HEAD: Option<&'static Commit>,
}

impl Git {
    pub fn new(name: String) -> Self {
        let last_commit_id = 0;
        Git {
            last_commit_id,
            name,
            HEAD: None,
        }
    }

    pub fn commit(&mut self, message: String) -> Commit {
        self.last_commit_id += 1;
        let commit = Commit::new(self.last_commit_id, self.HEAD, message);
        self.HEAD = Some(&commit);

        commit
    }

    pub fn log(&self) -> Vec<Option<&'static Commit>> {
        let mut history: Vec<Option<&'static Commit>> = Vec::new();

        let mut commit = self.HEAD;
        loop {
            match commit {
                Some(v) => {
                    history.push(v.parent);
                    commit = v.parent;
                }
                None => break,
            }
        }

        history
    }

    pub fn echo(&self) {
        println!("{}, {}", self.last_commit_id, self.name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn success() {
        let mut repo = Git::new("test".to_string());
        repo.commit("first commit".to_string());
        repo.commit("second commit".to_string());

        let logs = repo.log();
        assert_eq!(logs.len(), 2);
        assert_eq!(logs[0].last_commit_id, 0);
        assert_eq!(logs[1].last_commit_id, 1);
    }
}
