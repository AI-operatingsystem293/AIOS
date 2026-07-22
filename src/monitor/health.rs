use super::metrics::AgentMetrics;

#[derive(Debug)]
pub enum Health {

    Excellent,

    Good,

    Busy,

    Unhealthy,

    Offline,
}

pub struct HealthChecker;

impl HealthChecker {

    pub fn status(
        metrics: &AgentMetrics,
    ) -> Health {

        if metrics.crashes > 5 {

            return Health::Offline;
        }

        if metrics.running_tasks > 20 {

            return Health::Busy;
        }

        if metrics.tasks_failed > metrics.tasks_completed {

            return Health::Unhealthy;
        }

        if metrics.average_latency_ms < 200 {

            return Health::Excellent;
        }

        Health::Good
    }
}
