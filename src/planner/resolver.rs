use crate::capability::registry::ProviderRegistry;

pub struct CapabilityResolver<'a> {
    registry: &'a ProviderRegistry,
}

impl<'a> CapabilityResolver<'a> {

    pub fn new(
        registry: &'a ProviderRegistry,
    ) -> Self {

        Self {
            registry,
        }
    }

    pub fn resolve(
        &self,
        capability: &str,
    ) -> Option<String> {

        self.registry
            .best_provider(capability)
            .map(|provider| provider.agent_id.clone())
    }

    pub fn can_resolve(
        &self,
        capability: &str,
    ) -> bool {

        self.registry
            .best_provider(capability)
            .is_some()
    }
}
