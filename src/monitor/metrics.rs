#[derive(Clone, Debug)]
pub struct AgentMetrics {

    pub tasks_completed: u64,

    pub tasks_failed: u64,

    pub total_latency_ms: u64,

    pub average_latency_ms: u64,

    pub running_tasks: u32,

    pub crashes: u64,

    pub last_seen: u64,
}

impl AgentMetrics {

    pub fn new() -> Self {

        Self {

            tasks_completed: 0,

            tasks_failed: 0,

            total_latency_ms: 0,

            average_latency_ms: 0,

            running_tasks: 0,

            crashes: 0,

            last_seen: 0,
        }
    }

    pub fn record_success(
        &mut self,
        latency_ms: u64,
    ) {

        self.tasks_completed += 1;

        self.total_latency_ms += latency_ms;

        self.average_latency_ms =
            self.total_latency_ms
                / self.tasks_completed;
    }

    pub fn record_failure(
        &mut self,
    ) {

        self.tasks_failed += 1;
    }
}
