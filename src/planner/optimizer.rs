use crate::task::task::Task;

pub struct PlanOptimizer;

impl PlanOptimizer {
    pub fn new() -> Self {
        Self
    }

    pub fn optimize(
        &self,
        _tasks: &mut Vec<Task>,
    ) {
        // Future:
        // Remove duplicate tasks.
        // Merge compatible tasks.
        // Improve execution order.
    }
}
