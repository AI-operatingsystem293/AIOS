use crate::planner::graph::{TaskGraph, TaskNode};

pub struct ExecutionCoordinator;

impl ExecutionCoordinator {
    pub fn new() -> Self {
        Self
    }

    pub fn ready_tasks(
        &self,
        graph: &TaskGraph,
        completed: &[u64],
    ) -> Vec<TaskNode> {

        let mut ready = Vec::new();

        for node in graph.nodes() {

            let mut satisfied = true;

            for dep in &node.dependencies {

                if !completed.contains(dep) {
                    satisfied = false;
                    break;
                }
            }

            if satisfied {
                ready.push(node.clone());
            }
        }

        ready
    }
}
