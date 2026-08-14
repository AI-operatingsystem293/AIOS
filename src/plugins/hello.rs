use crate::sdk::{
    agent::Agent, manifest::AgentManifest, plugin::AgentPlugin, request::AgentRequest,
    response::AgentResponse,
};

pub struct HelloAgent;

impl HelloAgent {
    pub fn new() -> Self {
        Self
    }
}

impl Agent for HelloAgent {
    fn manifest(&self) -> AgentManifest {
        AgentManifest::new()
            .name("Hello Agent")
            .version("1.0.0")
            .author("AIOS")
            .description("Example AIOS plugin agent")
            .capability("hello")
    }

    fn execute(&mut self, request: AgentRequest) -> AgentResponse {
        let output = format!("Hello from plugin: {}", request.input);

        AgentResponse::success(request.task_id, &output)
    }
}

pub struct HelloPlugin;

impl HelloPlugin {
    pub fn new() -> Self {
        Self
    }
}

impl AgentPlugin for HelloPlugin {
    fn manifest(&self) -> AgentManifest {
        AgentManifest::new()
            .name("Hello Plugin")
            .version("1.0.0")
            .author("AIOS")
            .description("Example AIOS plugin")
            .capability("hello")
    }

    fn create(&self) -> Box<dyn Agent> {
        Box::new(HelloAgent::new())
    }
}
