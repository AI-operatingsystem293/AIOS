use crate::capability::{
    provider::Provider,
    score::CapabilityScore,
};

use super::{
    decision::SchedulingDecision,
    strategy::SchedulingStrategy,
};

pub struct SchedulingPolicy {

    strategy: SchedulingStrategy,
}

impl SchedulingPolicy {

    pub fn new() -> Self {

        Self {

            strategy:
                SchedulingStrategy::HighestScore,
        }
    }

    pub fn select(
        &self,
        providers: &[Provider],
    ) -> Option<SchedulingDecision> {

        if providers.is_empty() {

            return None;
        }

        let mut best: Option<&Provider> = None;

        let mut best_score = 0;

        for provider in providers {

            let score =
                CapabilityScore::calculate(provider);

            if score > best_score {

                best_score = score;

                best = Some(provider);
            }
        }

        let provider = best?;

        Some(

            SchedulingDecision::new(

                provider.agent_id.clone(),

                format!(
                    "Selected using {:?}",
                    self.strategy
                ),

                best_score,
            )
        )
    }
}
