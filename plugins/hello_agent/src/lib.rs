use aios::{
    sdk::{
        agent::Agent,
        request::AgentRequest,
        response::AgentResponse,
        manifest::AgentManifest,
    },
    sdk::plugin::AgentPlugin,
    plugin::manifest::PluginManifest,
};



pub struct DeveloperAgent;


impl DeveloperAgent {

    pub fn new() -> Self {
        Self
    }

}



impl Agent for DeveloperAgent {


    fn manifest(&self) -> AgentManifest {

        let mut m = AgentManifest::new();

        m.name =
            "Developer Agent".to_string();

        m.version =
            "1.0.0".to_string();

        m.author =
            "External Developer".to_string();

        m.description =
            "Third party AIOS agent".to_string();

        m.capabilities =
            vec![
                "developer".to_string()
            ];

        m
    }



    fn execute(
        &mut self,
        request: AgentRequest,
    ) -> AgentResponse {

        AgentResponse::success(
            request.task_id,
            "Developer plugin executed",
        )
    }

}



pub struct DeveloperPlugin;


impl DeveloperPlugin {

    pub fn new() -> Self {
        Self
    }

}



impl AgentPlugin for DeveloperPlugin {


    fn manifest(&self) -> PluginManifest {

        PluginManifest::new(
            "Developer Plugin",
            "1.0.0",
            "External Developer",
            "Custom AIOS plugin",
            vec![
                "developer"
            ],
        )

    }



    fn create(&self) -> Box<dyn Agent> {

        Box::new(
            DeveloperAgent::new()
        )

    }

}
