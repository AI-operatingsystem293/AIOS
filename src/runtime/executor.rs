use crate::{
    kernel::registry::AgentRegistry,
    runtime::result::RuntimeResult,
    task::task::Task,
};

pub struct Executor;

impl Executor {

    pub fn new() -> Self {
        Self
    }

    pub fn execute(
        &self,
        registry: &mut AgentRegistry,
        task: &Task,
    ) -> RuntimeResult {

        match registry.execute_command(
            &task.command,
            &task.input,
        ) {
            Some(output) => {
                RuntimeResult::success(
                    task.id,
                    output,
                )
            }

            None => {
                RuntimeResult::failed(
                    task.id,
                    format!(
                        "No compatible agent for '{}'",
                        task.command
                    ),
                )
            }
        }
    }
}
