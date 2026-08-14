use crate::{
    capability::registry::ProviderRegistry,
    planner::{goal::Goal, plan::Plan, rule::PlanningRule, rules::DefaultRule},
};

pub struct PlanningEngine {
    rules: Vec<Box<dyn PlanningRule>>,
}

impl PlanningEngine {
    pub fn new() -> Self {
        let mut engine = Self { rules: Vec::new() };

        engine.register(Box::new(DefaultRule::new()));

        engine
    }

    pub fn register(&mut self, rule: Box<dyn PlanningRule>) {
        self.rules.push(rule);
    }

    pub fn create_plan(&self, goal: &Goal, registry: &ProviderRegistry) -> Plan {
        for rule in &self.rules {
            if rule.matches(goal) {
                println!("Planner Rule -> {}", rule.name());

                return rule.build_plan_with_registry(goal, registry);
            }
        }

        Plan::new()
    }

    pub fn build_plan(&self, goal: &Goal, registry: &ProviderRegistry) -> Plan {
        self.create_plan(goal, registry)
    }

    pub fn print_plan(&self, goal: &Goal, registry: &ProviderRegistry) {
        let plan = self.build_plan(goal, registry);

        println!();
        println!("========== EXECUTION PLAN ==========");

        plan.list();

        println!("====================================");
    }
}
