use crate::capability::{provider::Provider, registry::ProviderRegistry};

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

    /*
    * Resolve a natural-language request.

    * We do not hardcode questions such as:
    *
    * "if contains rust -> web"
    *
    * Instead, the resolver compares the request with the
    * capabilities registered by agents.
    */
    pub fn resolve_request<'a>(
        &self,
        registry: &'a ProviderRegistry,
        request: &str,
    ) -> Option<&'a Provider> {
        registry
            .providers()
            .iter()
            .filter(|provider| provider.healthy)
            .max_by_key(|provider| Self::semantic_score(provider, request))
    }

    fn semantic_score(provider: &Provider, request: &str) -> u32 {
        let request = normalize(request);
        let capability = normalize(&provider.capability);

        let mut score = 0u32;

        /*
         * Exact capability match.
         */
        if request == capability {
            score += 1000;
        }

        /*
         * Capability appears directly in the request.
         */
        if request.split_whitespace().any(|word| word == capability) {
            score += 500;
        }

        /*
         * Shared words between request and capability.
         */
        score += token_overlap(&request, &capability) * 50;

        /*
         * Successful providers receive a small preference.
         */
        score += provider.success_count.min(20) as u32;

        /*
         * Failed providers are penalized.
         */
        score = score.saturating_sub(provider.failure_count as u32 * 5);

        /*
         * Busy providers receive a small penalty.
         */
        score = score.saturating_sub(provider.running_tasks * 2);

        score
    }

    pub fn list(&self, registry: &ProviderRegistry) {
        println!();
        println!("====== Capability Resolver ======");

        for provider in registry.providers() {
            println!("{} -> {}", provider.capability, provider.agent_id,);

            println!("  Success : {}", provider.success_count);

            println!("  Failure : {}", provider.failure_count);

            println!("  Healthy : {}", provider.healthy);
        }

        println!();
    }
}

fn normalize(value: &str) -> String {
    value
        .to_lowercase()
        .chars()
        .map(|c| {
            if c.is_alphanumeric() || c.is_whitespace() {
                c
            } else {
                ' '
            }
        })
        .collect()
}

fn token_overlap(a: &str, b: &str) -> u32 {
    use std::collections::HashSet;

    let b_tokens: HashSet<&str> = b.split_whitespace().collect();

    a.split_whitespace()
        .filter(|token| b_tokens.contains(token))
        .count() as u32
}
