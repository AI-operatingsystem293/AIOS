use crate::{
    capability::registry::ProviderRegistry,
    planner::{
        analyzer::GoalAnalyzer, decomposer::TaskDecomposer, dependency::DependencyBuilder,
        engine::PlanningEngine, goal::Goal, optimizer::PlanOptimizer, plan::Plan,
    },
    task::task::Task,
};

pub struct Planner {
    analyzer: GoalAnalyzer,
    decomposer: TaskDecomposer,
    dependency: DependencyBuilder,
    engine: PlanningEngine,
    optimizer: PlanOptimizer,
}

impl Planner {
    pub fn new() -> Self {
        Self {
            analyzer: GoalAnalyzer::new(),
            decomposer: TaskDecomposer::new(),
            dependency: DependencyBuilder::new(),
            optimizer: PlanOptimizer::new(),
            engine: PlanningEngine::new(),
        }
    }

    // Legacy planner.
    //
    // Kept for compatibility with older code.
    // The new execution path uses workflow().
    pub fn plan(&self, input: &str) -> Vec<Task> {
        let goal = self.analyzer.analyze(input);

        let mut tasks = self.decomposer.decompose(&goal);

        self.dependency.build(&mut tasks);

        self.optimizer.optimize(&mut tasks);

        tasks
    }

    // Capability-driven workflow planner.
    //
    // The planner receives the live ProviderRegistry.
    // Therefore newly registered agents become available
    // automatically.
    pub fn workflow(&self, input: &str, registry: &ProviderRegistry) -> Plan {
        let goal: Goal = self.analyzer.analyze(input);

        println!();
        println!("========== WORKFLOW ==========");

        println!("Goal: {}", goal.description);

        let plan = self.engine.build_plan(&goal, registry);

        plan.list();

        plan
    }

    pub fn explain(&self, input: &str, registry: &ProviderRegistry) {
        let plan = self.workflow(input, registry);

        println!();
        println!("Workflow Summary");

        println!("----------------");

        println!("Tasks : {}", plan.tasks.len());

        for task in &plan.tasks {
            println!("#{} {} [{}]", task.id, task.name, task.capability,);

            if !task.dependencies.is_empty() {
                println!("  Depends on {:?}", task.dependencies);
            }

            if !task.children.is_empty() {
                println!("  Children {:?}", task.children);
            }
        }

        println!("=============================");
    }
}
