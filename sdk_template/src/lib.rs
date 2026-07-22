use aios::sdk::{
    agent::Agent,
    context::AgentContext,
    manifest::AgentManifest,
    request::AgentRequest,
    response::AgentResponse,
};


pub struct MyAgent;


impl MyAgent {

    pub fn new() -> Self {
        Self
    }

}


impl Agent for MyAgent {


    fn manifest(
        &self,
    ) -> AgentManifest {

        AgentManifest::new()
            .name("My External Agent")
            .version("1.0.0")
            .author("Your Name")
            .description(
                "AIOS external developer agent"
            )
            .capability("my_command")

    }



    fn initialize(
        &mut self,
        _context: &AgentContext,
    ) -> Result<(), String> {

        println!(
            "My Agent initialized"
        );

        Ok(())

    }



    fn execute(
        &mut self,
        request: AgentRequest,
    ) -> AgentResponse {


        let output =
            format!(
                "External agent received: {}",
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
            "My Agent shutdown"
        );

        Ok(())

    }

}



#[no_mangle]
pub extern "C" fn create_plugin()
    -> *mut dyn Agent
{

    let agent =
        Box::new(
            MyAgent::new()
        );


    Box::into_raw(
        agent
    )

}
