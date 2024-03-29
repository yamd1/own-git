use super::branch::Branch;
use super::commit::Commit;

#[derive(Debug, Clone)]
pub struct Git {
    last_commit_id: u32,
    name: String,
    head: Branch,
    main: Branch,
    branches: Vec<Branch>,
}

impl Git {
    pub fn new(name: String) -> Self {
        let last_commit_id = 0;
        let main = Branch::new("main".to_string(), None);
        Git {
            last_commit_id,
            name,
            head: main.clone(),
            branches: vec![main.clone()],
            main,
        }
    }

    pub fn commit(&mut self, message: String) -> Commit {
        self.last_commit_id += 1;
        let commit = Commit::new(self.last_commit_id, self.head.commit.clone(), message);
        self.head.commit = Some(commit.clone());

        commit
    }

    pub fn log(&self) -> Vec<Option<Commit>> {
        let mut history: Vec<Option<Commit>> = Vec::new();

        let mut commit = self.head.commit.clone();
        loop {
            match commit {
                Some(v) => {
                    history.push(Some(v.clone()));
                    commit = v.parent.map(|p| p.as_ref().to_owned());
                }
                None => break,
            }
        }

        history
    }

    pub fn checkout(&mut self, name: String) -> Self {
        for v in self.branches.clone().into_iter() {
            if v.name == name {
                self.head = v;
                return self.to_owned();
            }
        }
        let new_branch = Branch::new(name, self.head.commit.clone());
        self.branches.push(new_branch.clone());
        self.head = new_branch;
        self.to_owned()
    }

    pub fn echo(&self) {
        println!("{}, {}", self.last_commit_id, self.name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn success_commit() {
        let mut repo = Git::new("test".to_string());
        repo.commit("first commit".to_string());
        repo.commit("second commit".to_string());

        let logs = repo.log();
        assert_eq!(logs.len(), 2);
        assert_eq!(logs[0].as_ref().unwrap().id, 2);
        assert_eq!(logs[1].as_ref().unwrap().id, 1);
    }

    #[test]
    fn success_checkout() {
        let mut repo = Git::new("test".to_string());
        repo.commit("initial commit".to_string());

        assert_eq!(repo.head.name, "main".to_string());
        repo.checkout("testing".to_string());
        assert_eq!(repo.head.name, "testing".to_string());
        repo.checkout("main".to_string());
        assert_eq!(repo.head.name, "main".to_string());
        repo.checkout("testing".to_string());
        assert_eq!(repo.head.name, "testing".to_string());
    }

    #[test]
    fn history_map() {
        fn history_to_id_mappser(mut history: Vec<Option<Commit>>) -> String {
            let mut ids: Vec<String> = vec![];
            for item in history.into_iter() {
                match item {
                    Some(v) => ids.push(v.id.to_string()),
                    None => break,
                }
            }
            ids.join("-")
        }
    }
}
