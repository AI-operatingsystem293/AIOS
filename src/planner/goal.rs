#[derive(Clone, Debug)]
pub struct Goal {
    pub id: u64,
    pub description: String,
}

impl Goal {
    pub fn new(id: u64, description: &str) -> Self {
        Self {
            id,
            description: description.to_string(),
        }
    }
}
