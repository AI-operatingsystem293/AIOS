use crate::sdk::{
    agent::Agent,
    manifest::AgentManifest,
    request::AgentRequest,
    response::AgentResponse,
};

pub struct EchoAgent;

impl EchoAgent {
    pub fn new() -> Self {
        Self
    }
}

impl Agent for EchoAgent {
    fn manifest(&self) -> AgentManifest {
        AgentManifest::new()
            .name("Echo Agent")
            .version("1.0.0")
            .author("AIOS")
            .description("Echoes input")
            .capability("echo")
    }

    fn execute(
        &mut self,
        request: AgentRequest,
    ) -> AgentResponse {
        AgentResponse::success(
            request.task_id,
            &request.input,
        )
    }
}
