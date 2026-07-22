use crate::{
    planner::{
        goal::Goal,
        plan::Plan,
    },
    task::task::Task,
};

pub trait PlanningRule {

    fn name(&self) -> &str;

    fn matches(
        &self,
        goal: &Goal,
    ) -> bool;

    // Legacy planner (kept for compatibility)
    fn expand(
        &self,
        goal: &Goal,
    ) -> Vec<Task>;

    // New planner API
    fn build_plan(
        &self,
        goal: &Goal,
    ) -> Plan {

        let tasks = self.expand(goal);

        let mut plan = Plan::new();

        for task in tasks {

            plan.add_task(
                crate::planner::task::PlanTask::new(
                    task.id,
                    &task.command,
                    &task.command,
                    &task.input,
                    task.priority,
                )
            );
        }

        plan
    }
}
