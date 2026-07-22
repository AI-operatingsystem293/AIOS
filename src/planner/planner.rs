use crate::planner::engine::PlanningEngine;

use crate::{
    planner::{
        analyzer::GoalAnalyzer,
        decomposer::TaskDecomposer,
        dependency::DependencyBuilder,
        goal::Goal,
        optimizer::PlanOptimizer,
        plan::Plan,
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



    // Legacy planner
    pub fn plan(
        &self,
        input: &str,
    ) -> Vec<Task> {


        let goal =
            self.analyzer.analyze(input);



        println!("Planner");

        println!("-------");

        println!(
            "{}",
            goal.description
        );



        let mut tasks =
            self.decomposer.decompose(&goal);



        self.dependency.build(
            &mut tasks
        );



        self.optimizer.optimize(
            &mut tasks
        );



        tasks

    }



    // New workflow planner
    pub fn workflow(
        &self,
        input: &str,
    ) -> Plan {


        let goal: Goal =
            self.analyzer.analyze(input);



        println!();

        println!(
            "========== WORKFLOW =========="
        );


        println!(
            "Goal: {}",
            goal.description
        );



        let plan =
            self.engine.build_plan(
                &goal
            );



        plan.list();



        plan

    }



    pub fn explain(
        &self,
        input: &str,
    ) {


        let plan =
            self.workflow(input);



        println!();

        println!(
            "Workflow Summary"
        );


        println!(
            "----------------"
        );



        println!(
            "Tasks : {}",
            plan.tasks.len()
        );



        for task in &plan.tasks {


            println!(
                "#{} {} [{}]",
                task.id,
                task.name,
                task.capability,
            );



            if !task.dependencies.is_empty() {

                println!(
                    "  Depends on {:?}",
                    task.dependencies
                );

            }



            if !task.children.is_empty() {

                println!(
                    "  Children {:?}",
                    task.children
                );

            }

        }



        println!(
            "=============================="
        );

    }

}
