use super::commit::Commit;

#[derive(Debug)]
pub struct Git {
    last_commit_id: u32,
    name: String,
}

impl Git {
    pub fn new(name: String) -> Self {
        let last_commit_id = 0;
        Git {
            last_commit_id,
            name,
        }
    }

    pub fn commit(&mut self, message: String) -> Commit {
        self.last_commit_id += 1;
        Commit::new(self.last_commit_id, message)
    }

    pub fn log(&self) -> Vec<Git> {
        let history: Vec<Git> = Vec::new();

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
