use crate::task::task::Task;

pub struct DependencyBuilder;

impl DependencyBuilder {
    pub fn new() -> Self {
        Self
    }

    pub fn build(
        &self,
        _tasks: &mut Vec<Task>,
    ) {
        // Future:
        // Analyze task dependencies.
        // Detect tasks that can execute in parallel.
        // Build execution graph.
    }
}
