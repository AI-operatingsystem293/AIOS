pub struct AgentContext {
    pub runtime_version: String,
    pub kernel_version: String,
}

impl AgentContext {

    pub fn new() -> Self {

        Self {
            runtime_version: "0.1.0".to_string(),
            kernel_version: "0.1.0".to_string(),
        }
    }
}
