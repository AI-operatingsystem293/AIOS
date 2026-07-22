use crate::{
    planner::{
        goal::Goal,
        plan::Plan,
        reasoner::Reasoner,
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

}



impl PlanningRule for DefaultRule {


    fn name(
        &self,
    ) -> &str {

        "default"

    }



    fn matches(
        &self,
        _goal: &Goal,
    ) -> bool {

        true

    }



    // Legacy planner
    fn expand(
        &self,
        goal: &Goal,
    ) -> Vec<Task> {


        let text =
            goal.description
                .to_lowercase();


        let mut tasks =
            Vec::new();


        let mut id: u64 = 1;


        let mut previous: Option<u64> =
            None;



        //
        // Dynamic fallback task generation
        //
        // No fixed "add", "multiply"
        // rules here.
        //

        let words: Vec<&str> =
            goal.description
                .split_whitespace()
                .collect();



        if !words.is_empty() {


            let command =
                words[0].to_string();


            let input =
                words
                    .iter()
                    .skip(1)
                    .cloned()
                    .collect::<Vec<_>>()
                    .join(" ");



            tasks.push(
                Task::new(
                    id,
                    previous,
                    command,
                    input,
                    5,
                )
            );


            previous =
                Some(id);


            id += 1;

        }



        //
        // Verification task
        //

        if !tasks.is_empty() {


            tasks.push(
                Task::new(
                    id,
                    previous,
                    "verify".to_string(),
                    "".to_string(),
                    1,
                )
            );

        }



        //
        // Empty goal fallback
        //

        if tasks.is_empty() {


            tasks.push(
                Task::new(
                    1,
                    None,
                    "answer".to_string(),
                    text,
                    1,
                )
            );

        }


        tasks

    }



    // Workflow planner
    fn build_plan(
        &self,
        goal: &Goal,
    ) -> Plan {


        let mut plan =
            Plan::new();



        let tasks =
            self.expand(goal);



        for task in tasks {


            plan.add_task(

                PlanTask::new(
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
