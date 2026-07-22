#[derive(Clone, Debug)]
pub struct AgentRequest {
    pub id: u64,
    pub task_id: u64,
    pub capability: String,
    pub input: String,
}

impl AgentRequest {

    pub fn new(
        id: u64,
        task_id: u64,
        capability: &str,
        input: &str,
    ) -> Self {

        Self {
            id,
            task_id,
            capability: capability.to_string(),
            input: input.to_string(),
        }
    }
}
