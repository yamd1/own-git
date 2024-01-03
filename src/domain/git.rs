pub struct Git {
    name: String,
}

impl Git {
    pub fn new(name: String) -> Self {
        Git { name }
    }

    pub fn echo(&self) -> &str {
        &self.name
    }
}
