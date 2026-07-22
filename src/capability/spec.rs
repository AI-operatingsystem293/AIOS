use super::{
    input::InputType,
    output::OutputType,
};

#[derive(Clone, Debug)]
pub struct CapabilitySpec {
    pub name: String,
    pub description: String,
    pub version: String,
    pub input: InputType,
    pub output: OutputType,

    pub permissions: Vec<String>,

    // Agents that implement this capability
    pub agents: Vec<String>,
}

impl CapabilitySpec {
    pub fn new(
        name: &str,
        description: &str,
        input: InputType,
        output: OutputType,
    ) -> Self {
        Self {
            name: name.to_string(),
            description: description.to_string(),
            version: "1.0.0".to_string(),
            input,
            output,
            permissions: Vec::new(),
            agents: Vec::new(),
        }
    }

    pub fn permission(
        mut self,
        permission: &str,
    ) -> Self {
        self.permissions.push(permission.to_string());
        self
    }

    pub fn agent(
        mut self,
        agent: &str,
    ) -> Self {
        self.agents.push(agent.to_string());
        self
    }
}
