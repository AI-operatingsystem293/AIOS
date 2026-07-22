#[derive(Clone, Debug)]
pub struct Provider {

    pub agent_id: String,

    pub capability: String,

    pub success_count: u64,

    pub failure_count: u64,

    pub average_latency_ms: u64,

    pub running_tasks: u32,

    pub healthy: bool,
}

impl Provider {

    pub fn new(
        agent: &str,
        capability: &str,
    ) -> Self {

        Self {

            agent_id: agent.to_string(),

            capability: capability.to_string(),

            success_count: 0,

            failure_count: 0,

            average_latency_ms: 0,

            running_tasks: 0,

            healthy: true,
        }
    }
}
