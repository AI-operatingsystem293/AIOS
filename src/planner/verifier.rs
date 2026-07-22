use crate::{
    capability::registry::ProviderRegistry,
    task::task::Task,
};

pub struct PlanVerifier<'a> {
    registry: &'a ProviderRegistry,
}

impl<'a> PlanVerifier<'a> {

    pub fn new(
        registry: &'a ProviderRegistry,
    ) -> Self {

        Self {
            registry,
        }
    }

    pub fn verify(
        &self,
        tasks: &[Task],
    ) -> Result<(), String> {

        for task in tasks {

            if self
                .registry
                .best_provider(&task.command)
                .is_none()
            {

                return Err(format!(
                    "No provider for '{}'",
                    task.command
                ));
            }
        }

        Ok(())
    }
}
