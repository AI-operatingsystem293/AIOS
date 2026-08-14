use crate::{
    capability::registry::ProviderRegistry,
    planner::{
        goal::Goal,
        plan::Plan,
        reasoner::{PlanningDecision, Reasoner},
        rule::PlanningRule,
        task::PlanTask,
    },
    task::task::Task,
};

pub struct DefaultRule {
    reasoner: Reasoner,
}

impl DefaultRule {
    pub fn new() -> Self {
        Self {
            reasoner: Reasoner::new(),
        }
    }

    fn decisions(
        &self,
        goal: &Goal,
        registry: &ProviderRegistry,
    ) -> Vec<PlanningDecision> {
        self.reasoner.reason(
            goal,
            registry.providers(),
        )
    }
}

impl PlanningRule for DefaultRule {
    fn name(&self) -> &str {
        "capability-driven"
    }

    fn matches(&self, _goal: &Goal) -> bool {
        true
    }

    fn expand(&self, goal: &Goal) -> Vec<Task> {
        vec![
            Task::new(
                1,
                None,
                "general_reasoning".to_string(),
                goal.description.clone(),
                1,
            ),
        ]
    }

    fn build_plan_with_registry(
        &self,
        goal: &Goal,
        registry: &ProviderRegistry,
    ) -> Plan {
        let decisions = self.decisions(goal, registry);

        let mut plan = Plan::new();

        if decisions.is_empty() {
            return plan;
        }

        for (index, decision) in decisions.into_iter().enumerate() {
            let id = (index + 1) as u64;

            plan.add_task(PlanTask::new(
                id,
                &decision.capability,
                &decision.capability,
                &decision.input,
                decision.priority,
            ));
        }

        plan
    }

    fn build_plan(&self, goal: &Goal) -> Plan {
        let mut plan = Plan::new();

        let task = Task::new(
            1,
            None,
            "general_reasoning".to_string(),
            goal.description.clone(),
            1,
        );

        plan.add_task(PlanTask::new(
            task.id,
            &task.command,
            &task.command,
            &task.input,
            task.priority,
        ));

        plan
    }
}
