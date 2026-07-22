#[derive(Clone, Debug)]
pub struct AgentResponse {
    pub task_id: u64,
    pub success: bool,
    pub output: String,
}

impl AgentResponse {

    pub fn success(
        task_id: u64,
        output: &str,
    ) -> Self {

        Self {
            task_id,
            success: true,
            output: output.to_string(),
        }
    }

    pub fn error(
        task_id: u64,
        message: &str,
    ) -> Self {

        Self {
            task_id,
            success: false,
            output: message.to_string(),
        }
    }
}
