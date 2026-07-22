#[derive(Clone, Debug)]
pub struct AgentManifest {

    // Identity
    pub id: String,
    pub name: String,
    pub version: String,

    // SDK
    pub sdk_version: String,
    pub requires_aios: String,

    // Executable
    pub executable: String,

    // Metadata
    pub author: String,
    pub license: String,
    pub description: String,

    // Resources
    pub min_memory: String,

    // Security
    pub permissions: Vec<String>,

    // Features
    pub capabilities: Vec<String>,
}

impl AgentManifest {

    pub fn new() -> Self {

        Self {

            id: String::new(),
            name: String::new(),
            version: String::new(),

            sdk_version: "0.1.0".to_string(),
            requires_aios: ">=0.1.0".to_string(),

            executable: String::new(),

            author: String::new(),
            license: String::new(),
            description: String::new(),

            min_memory: "32MB".to_string(),

            permissions: Vec::new(),

            capabilities: Vec::new(),
        }
    }

    pub fn capability(
        mut self,
        capability: &str,
    ) -> Self {

        self.capabilities
            .push(capability.to_string());

        self
    }

    pub fn permission(
        mut self,
        permission: &str,
    ) -> Self {

        self.permissions
            .push(permission.to_string());

        self
    }
}
