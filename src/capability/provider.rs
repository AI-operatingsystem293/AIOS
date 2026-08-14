#[derive(Clone, Debug)]
pub struct Provider {
    pub agent_id: String,
    pub capability: String,

    // Semantic metadata used by the resolver.
    pub description: String,
    pub keywords: Vec<String>,

    pub success_count: u64,
    pub failure_count: u64,
    pub average_latency_ms: u64,
    pub running_tasks: u32,
    pub healthy: bool,
}

impl Provider {
    pub fn new(agent: &str, capability: &str) -> Self {
        Self {
            agent_id: agent.to_string(),
            capability: capability.to_string(),
            description: String::new(),
            keywords: Vec::new(),
            success_count: 0,
            failure_count: 0,
            average_latency_ms: 0,
            running_tasks: 0,
            healthy: true,
        }
    }

    pub fn describe(mut self, description: &str) -> Self {
        self.description = description.to_string();
        self
    }

    pub fn keywords<I, S>(mut self, keywords: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.keywords = keywords.into_iter().map(Into::into).collect();
        self
    }
}
