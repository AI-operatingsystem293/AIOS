#[derive(Clone, Debug)]
pub struct AgentResult {
    pub success: bool,
    pub output: String,
}

impl AgentResult {
    pub fn success(output: &str) -> Self {
        Self {
            success: true,
            output: output.to_string(),
        }
    }

    pub fn failure(output: &str) -> Self {
        Self {
            success: false,
            output: output.to_string(),
        }
    }
}
