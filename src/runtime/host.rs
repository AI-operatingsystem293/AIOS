use crate::runtime::{
    discovery::AgentDiscovery,
    manifest::AgentManifest,
};

pub struct AgentHost {
    manifests: Vec<AgentManifest>,
}

impl AgentHost {

    pub fn new() -> Self {

        Self {
            manifests: Vec::new(),
        }
    }

    pub fn discover(
        &mut self,
        directory: &str,
    ) {

        let folders =
            AgentDiscovery::discover(directory);

        println!();
        println!("Scanning {}", directory);

        for folder in folders {

            println!("Found Agent: {}", folder);

            self.manifests.push(

                AgentManifest::new(
                    &folder,
                    &folder,
                    "0.1.0",
                    "",
                )
            );
        }
    }

    pub fn list(&self) {

        println!();
        println!("====== EXTERNAL AGENTS ======");

        for agent in &self.manifests {

            println!(
                "{} {}",
                agent.name,
                agent.version
            );
        }

        println!("=============================");
    }
}
