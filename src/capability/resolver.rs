use crate::capability::{
    provider::Provider,
    registry::ProviderRegistry,
};

pub struct CapabilityResolver;

impl CapabilityResolver {
    pub fn new() -> Self {
        Self
    }

    pub fn resolve_best<'a>(
        &self,
        registry: &'a ProviderRegistry,
        capability: &str,
    ) -> Option<&'a Provider> {
        registry.best_provider(capability)
    }

    pub fn resolve<'a>(
        &self,
        registry: &'a ProviderRegistry,
        capability: &str,
    ) -> Option<&'a Provider> {
        self.resolve_best(registry, capability)
    }

    pub fn providers<'a>(
        &self,
        registry: &'a ProviderRegistry,
        capability: &str,
    ) -> Vec<&'a Provider> {
        registry.providers_for(capability)
    }

    pub fn list(
        &self,
        registry: &ProviderRegistry,
    ) {
        println!();
        println!("====== Capability Resolver ======");

        for provider in registry.providers() {
            println!(
                "{} -> {}",
                provider.capability,
                provider.agent_id,
            );
        }

        println!();
    }
}
