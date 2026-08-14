use crate::{
    capability::registry::ProviderRegistry,
    planner::{goal::Goal, plan::Plan},
    task::task::Task,
};

pub trait PlanningRule {
    fn name(&self) -> &str;

    fn matches(&self, goal: &Goal) -> bool;

    fn expand(&self, goal: &Goal) -> Vec<Task>;

    fn build_plan(&self, goal: &Goal) -> Plan {
        let tasks = self.expand(goal);

        let mut plan = Plan::new();

        for task in tasks {
            plan.add_task(crate::planner::task::PlanTask::new(
                task.id,
                &task.command,
                &task.command,
                &task.input,
                task.priority,
            ));
        }

        plan
    }

    fn build_plan_with_registry(&self, goal: &Goal, _registry: &ProviderRegistry) -> Plan {
        self.build_plan(goal)
    }
}
