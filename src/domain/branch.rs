use super::commit::Commit;

#[derive(Debug, Clone)]
pub struct Branch {
    pub name: String,
    pub commit: Option<Commit>,
}

impl Branch {
    pub fn new(name: String, commit: Option<Commit>) -> Self {
        Branch { name, commit }
    }
}
