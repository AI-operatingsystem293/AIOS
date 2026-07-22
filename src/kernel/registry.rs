use crate::{
    capability::{
        provider::Provider,
        registry::ProviderRegistry,
    },
    sdk::{
        agent::Agent,
        request::AgentRequest,
    },
};

pub struct AgentRegistry {
    agents: Vec<Box<dyn Agent>>,
    providers: ProviderRegistry,
    libraries: Vec<libloading::Library>,
}

impl AgentRegistry {

    pub fn new() -> Self {
        Self {
           agents: Vec::new(),
           providers: ProviderRegistry::new(),
           libraries: Vec::new(),
        }
    }

    pub fn keep_library(
        &mut self,
        library: libloading::Library,
    ) {
        self.libraries.push(library);
    }

    pub fn register(
        &mut self,
        agent: Box<dyn Agent>,
    ) {
        let manifest = agent.manifest();

        for capability in &manifest.capabilities {
            self.providers.register(
                Provider::new(
                    &manifest.name,
                    capability,
                ),
            );
        }

        self.agents.push(agent);
    }

    pub fn start(&mut self) {
        use crate::agents::{
            echo::EchoAgent,
            math::MathAgent,
        };

        self.register(Box::new(EchoAgent::new()));
        self.register(Box::new(MathAgent::new()));

        println!("✓ {} agent(s) loaded", self.agents.len());

        println!("✓ {} provider(s) registered",
            self.providers.providers().len(),
        );
    }

    pub fn execute_command(
        &mut self,
        command: &str,
        input: &str,
    ) -> Option<String> {

        for agent in self.agents.iter_mut() {

            let manifest = agent.manifest();

            if manifest
                .capabilities
                .iter()
                .any(|c| c == command)
            {

                let request = AgentRequest::new(
                    0,
                    0,
                    command,
                    input,
                );

                let response =
                    agent.execute(request);

                if response.success {
                    self.providers
                        .mark_success(&manifest.name);
                } else {
                    self.providers
                        .mark_failure(&manifest.name);
                }

                return Some(response.output);
            }
        }

        None
    }

    pub fn providers(
        &self,
    ) -> &ProviderRegistry {
        &self.providers
    }

    pub fn providers_mut(
        &mut self,
    ) -> &mut ProviderRegistry {
        &mut self.providers
    }

    pub fn list_agents(&self) {

        println!();
        println!("========== AGENTS ==========");

        for agent in &self.agents {

            let manifest = agent.manifest();

            println!(
                "{} v{}",
                manifest.name,
                manifest.version,
            );

            println!(
                "Author: {}",
                manifest.author,
            );

            println!(
                "{}",
                manifest.description,
            );

            println!("Capabilities:");

            for capability in &manifest.capabilities {
                println!("  - {}", capability);
            }

            println!();
        }
    }

    pub fn list_providers(&self) {
        self.providers.list();
    }

    pub fn find_agent_name(
        &self,
        capability: &str,
    ) -> Option<String> {

        self.providers
            .best_provider(capability)
            .map(|p| p.agent_id.clone())
    }
}
