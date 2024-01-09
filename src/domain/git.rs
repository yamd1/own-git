use super::commit::Commit;

#[derive(Debug, Clone)]
pub struct Git {
    last_commit_id: u32,
    name: String,
    HEAD: Option<Commit>,
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
        let commit = Commit::new(self.last_commit_id, self.HEAD.clone(), message);
        self.HEAD = Some(commit.clone());

        commit
    }

    pub fn log(&self) -> Vec<Option<Commit>> {
        let mut history: Vec<Option<Commit>> = Vec::new();

        let mut commit = self.HEAD.clone();
        loop {
            match commit {
                Some(v) => {
                    let resolved = v.parent.map(|p| p.as_ref().to_owned());
                    history.push(resolved.clone());
                    commit = resolved;
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
        assert_eq!(logs[0].as_ref().unwrap().id, 0);
        assert_eq!(logs[1].as_ref().unwrap().id, 1);
    }
}
