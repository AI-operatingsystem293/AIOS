use crate::capability::provider::Provider;

use super::{policy::RecoveryPolicy, result::RecoveryResult};

pub struct RecoveryEngine;

impl RecoveryEngine {
    pub fn new() -> Self {
        Self
    }

    pub fn recover(
        &self,
        providers: &[Provider],
        capability: &str,
        failed_agent: &str,
    ) -> RecoveryResult {
        println!();
        println!("========== RECOVERY ENGINE ==========");

        let policy = self.choose_policy(providers, capability, failed_agent);

        match policy {
            RecoveryPolicy::RetrySameAgent => {
                println!("Recovery: retrying agent '{}'", failed_agent);

                RecoveryResult::Retry
            }

            RecoveryPolicy::TryAnotherAgent => {
                println!("Recovery: switching to another '{}' provider.", capability);

                if let Some(provider) =
                    self.find_backup_provider(providers, capability, failed_agent)
                {
                    println!("Selected backup agent: {}", provider.agent_id);

                    RecoveryResult::Recovered
                } else {
                    println!(
                        "No compatible backup provider for capability '{}'.",
                        capability
                    );

                    RecoveryResult::Failed
                }
            }

            RecoveryPolicy::Replan => {
                println!("Recovery: requesting replanning.");

                RecoveryResult::Replanned
            }

            RecoveryPolicy::Abort => {
                println!("Recovery: aborting execution.");

                RecoveryResult::Failed
            }
        }
    }

    fn choose_policy(
        &self,
        providers: &[Provider],
        capability: &str,
        failed_agent: &str,
    ) -> RecoveryPolicy {
        let healthy_backup = providers.iter().any(|provider| {
            provider.agent_id != failed_agent
                && provider.capability == capability
                && provider.healthy
        });

        if healthy_backup {
            return RecoveryPolicy::TryAnotherAgent;
        }

        let same_agent = providers.iter().find(|provider| {
            provider.agent_id == failed_agent && provider.capability == capability
        });

        if let Some(provider) = same_agent {
            if provider.healthy {
                return RecoveryPolicy::RetrySameAgent;
            }
        }

        RecoveryPolicy::Replan
    }

    fn find_backup_provider<'a>(
        &self,
        providers: &'a [Provider],
        capability: &str,
        failed_agent: &str,
    ) -> Option<&'a Provider> {
        providers
            .iter()
            .filter(|provider| {
                provider.agent_id != failed_agent
                    && provider.capability == capability
                    && provider.healthy
            })
            .min_by_key(|provider| {
                (
                    provider.running_tasks,
                    provider.average_latency_ms,
                    provider.failure_count,
                )
            })
    }
}
