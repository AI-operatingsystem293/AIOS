use crate::sdk::{
    context::AgentContext,
    manifest::AgentManifest,
    request::AgentRequest,
    response::AgentResponse,
};
pub trait Agent: Send + Sync {
    /// Metadata about this agent
    fn manifest(&self) -> AgentManifest;

    /// Called once when the agent starts
    fn initialize(
        &mut self,
        _context: &AgentContext,
    ) -> Result<(), String> {
        Ok(())
    }

    /// Execute one request
    fn execute(
        &mut self,
        request: AgentRequest,
    ) -> AgentResponse;

    /// Called before shutdown
    fn shutdown(
        &mut self,
    ) -> Result<(), String> {
        Ok(())
    }
}
