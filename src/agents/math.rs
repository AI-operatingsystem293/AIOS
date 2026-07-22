use crate::sdk::{
    agent::Agent,
    manifest::AgentManifest,
    request::AgentRequest,
    response::AgentResponse,
};

pub struct MathAgent;

impl MathAgent {
    pub fn new() -> Self {
        Self
    }
}

impl Agent for MathAgent {
    fn manifest(&self) -> AgentManifest {
        AgentManifest::new()
            .name("Math Agent")
            .version("1.0.0")
            .author("AIOS")
            .description("Basic Math")
            .capability("add")
    }

    fn execute(
        &mut self,
        request: AgentRequest,
    ) -> AgentResponse {
        let nums: Vec<i32> = request
            .input
            .split_whitespace()
            .filter_map(|x| x.parse::<i32>().ok())
            .collect();

        if nums.len() != 2 {
            return AgentResponse::error(
                request.task_id,
                "Usage: add <a> <b>",
            );
        }

        let result = (nums[0] + nums[1]).to_string();

        AgentResponse::success(
            request.task_id,
            &result,
        )
    }
}
