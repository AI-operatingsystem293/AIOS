#[derive(Clone, Debug)]
pub struct AgentManifest {
    pub name: String,
    pub version: String,
    pub author: String,
    pub description: String,
    pub capabilities: Vec<String>,
}

impl AgentManifest {
    pub fn new() -> Self {
        Self {
            name: String::new(),
            version: String::new(),
            author: String::new(),
            description: String::new(),
            capabilities: Vec::new(),
        }
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = name.to_string();
        self
    }

    pub fn version(mut self, version: &str) -> Self {
        self.version = version.to_string();
        self
    }

    pub fn author(mut self, author: &str) -> Self {
        self.author = author.to_string();
        self
    }

    pub fn description(mut self, description: &str) -> Self {
        self.description = description.to_string();
        self
    }

    pub fn capability(mut self, capability: &str) -> Self {
        self.capabilities.push(capability.to_string());
        self
    }
}
