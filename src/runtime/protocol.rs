use crate::runtime::message::RuntimeMessage;

pub trait AgentRuntime {
    fn name(&self) -> &str;

    fn version(&self) -> &str;

    fn capabilities(&self) -> Vec<String>;

    fn handle(
        &mut self,
        message: RuntimeMessage,
    ) -> RuntimeMessage;
}
