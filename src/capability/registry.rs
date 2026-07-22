use crate::capability::{
    provider::Provider,
    score::CapabilityScore,
};

pub struct ProviderRegistry {
    providers: Vec<Provider>,
}

impl ProviderRegistry {
    pub fn new() -> Self {
        Self {
            providers: Vec::new(),
        }
    }

    pub fn register(
        &mut self,
        provider: Provider,
    ) {
        self.providers.push(provider);
    }

    pub fn providers(&self) -> &[Provider] {
        &self.providers
    }

    pub fn providers_for(
        &self,
        capability: &str,
    ) -> Vec<&Provider> {

        self.providers
            .iter()
            .filter(|p| {
                p.capability == capability
            })
            .collect()
    }

    pub fn best_provider(
        &self,
        capability: &str,
    ) -> Option<&Provider> {

        self.providers
            .iter()
            .filter(|p| {
                p.capability == capability
                    && p.healthy
            })
            .max_by_key(|p| {
                CapabilityScore::calculate(p)
            })
    }

    pub fn mark_success(
        &mut self,
        agent: &str,
    ) {

        if let Some(provider) =
            self.providers
                .iter_mut()
                .find(|p| p.agent_id == agent)
        {
            provider.success_count += 1;
        }
    }

    pub fn mark_failure(
        &mut self,
        agent: &str,
    ) {

        if let Some(provider) =
            self.providers
                .iter_mut()
                .find(|p| p.agent_id == agent)
        {
            provider.failure_count += 1;
        }
    }

    pub fn list(&self) {

        println!();
        println!("========== PROVIDERS ==========");

        for provider in &self.providers {

            println!(
                "{} [{}]",
                provider.agent_id,
                provider.capability,
            );

            println!(
                " Score   : {}",
                CapabilityScore::calculate(provider),
            );

            println!(
                " Success : {}",
                provider.success_count,
            );

            println!(
                " Failure : {}",
                provider.failure_count,
            );

            println!(
                " Healthy : {}",
                provider.healthy,
            );

            println!();
        }
    }
}
