use std::collections::HashMap;

pub struct CapabilityManager {
    agents: HashMap<String, Vec<String>>,
}

impl CapabilityManager {
    pub fn new() -> Self {
        Self {
            agents: HashMap::new(),
        }
    }

    pub fn register(
        &mut self,
        agent: &str,
        capabilities: Vec<&str>,
    ) {
        self.agents.insert(
            agent.to_string(),
            capabilities
                .into_iter()
                .map(|s| s.to_string())
                .collect(),
        );
    }

    pub fn find(&self, capability: &str) -> Option<String> {
        for (agent, caps) in &self.agents {
            if caps.iter().any(|c| c == capability) {
                return Some(agent.clone());
            }
        }

        None
    }

    pub fn list(&self) {
        println!();
        println!("========== CAPABILITIES ==========");

        for (agent, caps) in &self.agents {
            println!("Agent: {}", agent);

            for cap in caps {
                println!("  - {}", cap);
            }

            println!();
        }
    }
}
