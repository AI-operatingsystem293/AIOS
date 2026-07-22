use super::provider::Provider;

pub struct CapabilityScore;

impl CapabilityScore {
    pub fn calculate(
        provider: &Provider,
    ) -> u32 {

        let mut score: u32 = 100;

        score = score.saturating_sub(
            provider.failure_count as u32 * 5,
        );

        score = score.saturating_sub(
            (provider.average_latency_ms / 100) as u32,
        );

        score = score.saturating_sub(
            provider.running_tasks * 2,
        );

        if !provider.healthy {
            score = 0;
        }

        score
    }
}
