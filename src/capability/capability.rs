#[derive(Clone, Debug)]
pub struct Capability {
    pub name: String,
}

impl Capability {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}
