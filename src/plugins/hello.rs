use crate::{
    sdk::{
        agent::Agent,
        manifest::AgentManifest,
        request::AgentRequest,
        response::AgentResponse,
    },
    sdk::plugin::AgentPlugin,
    plugin::manifest::PluginManifest,
};


pub struct HelloAgent;


impl HelloAgent {

    pub fn new() -> Self {
        Self
    }

}



impl Agent for HelloAgent {


    fn manifest(
        &self,
    ) -> AgentManifest {

        let mut manifest = AgentManifest::new();

        manifest.name =
            "Hello Agent".to_string();

        manifest.version =
            "1.0.0".to_string();

        manifest.author =
            "AIOS".to_string();

        manifest.description =
            "Example plugin agent".to_string();

        manifest.capabilities =
            vec![
                "hello".to_string()
            ];

        manifest
    }



    fn execute(
        &mut self,
        request: AgentRequest,
    ) -> AgentResponse {


        let output =
            format!(
                "Hello from plugin: {}",
                request.input
            );


        AgentResponse::success(
            request.task_id,
            &output,
        )

    }

}



pub struct HelloPlugin;



impl HelloPlugin {

    pub fn new() -> Self {
        Self
    }

}



impl AgentPlugin for HelloPlugin {


    fn manifest(
        &self,
    ) -> PluginManifest {

        PluginManifest::new(
            "Hello Plugin",
            "1.0.0",
            "AIOS",
            "Example AIOS plugin",
            vec![
                "hello"
            ],
        )

    }



    fn create(
        &self,
    ) -> Box<dyn Agent> {

        Box::new(
            HelloAgent::new()
        )

    }

}
