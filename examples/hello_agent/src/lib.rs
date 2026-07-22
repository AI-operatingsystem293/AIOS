use aios::sdk::{
    agent::Agent,
    context::AgentContext,
    manifest::AgentManifest,
    request::AgentRequest,
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
            .name("Hello External Agent")
            .version("1.0.0")
            .author("External Developer")
            .description(
                "Example external AIOS agent"
            )
            .capability("hello")

    }


    fn initialize(
        &mut self,
        _context: &AgentContext,
    ) -> Result<(), String> {

        println!(
            "Hello Agent initialized"
        );

        Ok(())

    }


    fn execute(
        &mut self,
        request: AgentRequest,
    ) -> AgentResponse {

        let output =
            format!(
                "Hello from external agent: {}",
                request.input
            );


        AgentResponse::success(
            request.task_id,
            &output,
        )

    }


        fn shutdown(
        &mut self,
    ) -> Result<(), String> {

        println!(
            "Hello Agent shutdown"
        );

        Ok(())

    }

}


aios::export_agent_plugin!(HelloAgent);
