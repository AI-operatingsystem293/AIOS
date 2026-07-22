use crate::{
    planner::{
        goal::Goal,
        plan::Plan,
        rule::PlanningRule,
        rules::DefaultRule,
    },
};


pub struct PlanningEngine {

    rules: Vec<Box<dyn PlanningRule>>,

}



impl PlanningEngine {


    pub fn new() -> Self {

        let mut engine = Self {

            rules: Vec::new(),

        };


        engine.register(
            Box::new(DefaultRule::new())
        );


        engine

    }



    pub fn register(
        &mut self,
        rule: Box<dyn PlanningRule>,
    ) {

        self.rules.push(rule);

    }



    pub fn create_plan(
        &self,
        goal: &Goal,
    ) -> Plan {


        for rule in &self.rules {


            if rule.matches(goal) {


                println!(
                    "Planner Rule -> {}",
                    rule.name()
                );


                return rule.build_plan(goal);

            }

        }



        Plan::new()

    }



    pub fn build_plan(
        &self,
        goal: &Goal,
    ) -> Plan {

        self.create_plan(goal)

    }



    pub fn print_plan(
        &self,
        goal: &Goal,
    ) {


        let plan =
            self.build_plan(goal);



        println!();

        println!(
            "========== EXECUTION PLAN =========="
        );


        plan.list();


        println!(
            "===================================="
        );

    }

}
