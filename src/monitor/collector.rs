use std::collections::HashMap;

use super::metrics::AgentMetrics;

pub struct MetricsCollector {

    metrics:
        HashMap<String, AgentMetrics>,
}

impl MetricsCollector {

    pub fn new() -> Self {

        Self {

            metrics:
                HashMap::new(),
        }
    }

    pub fn register(
        &mut self,
        agent: &str,
    ) {

        self.metrics
            .insert(
                agent.to_string(),
                AgentMetrics::new(),
            );
    }

    pub fn get(
        &self,
        agent: &str,
    ) -> Option<&AgentMetrics> {

        self.metrics.get(agent)
    }

    pub fn get_mut(
        &mut self,
        agent: &str,
    ) -> Option<&mut AgentMetrics> {

        self.metrics.get_mut(agent)
    }
}
