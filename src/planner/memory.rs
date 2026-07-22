use crate::task::task::Task;


#[derive(Clone)]
pub struct LearnedPlan {

    pub goal_pattern: String,

    pub commands: Vec<String>,

}



pub struct PlannerMemory {

    plans: Vec<LearnedPlan>,

}



impl PlannerMemory {


    pub fn new() -> Self {

        Self {
            plans: Vec::new(),
        }

    }



    pub fn find(
        &self,
        input: &str,
    ) -> Option<Vec<Task>> {


        for plan in &self.plans {


            if input.contains(
                &plan.goal_pattern
            ) {


                let mut tasks =
                    Vec::new();


                let mut parent =
                    None;


                let mut id =
                    1;



                for command in &plan.commands {


                    tasks.push(
                        Task::new(
                            id,
                            parent,
                            command.clone(),
                            input.to_string(),
                            1,
                        )
                    );


                    parent =
                        Some(id);


                    id += 1;

                }


                return Some(tasks);

            }

        }


        None

    }



    pub fn learn(
        &mut self,
        goal: &str,
        tasks: &[Task],
    ) {


        let commands =
            tasks
                .iter()
                .map(|task| task.command.clone())
                .collect();



        self.plans.push(
            LearnedPlan {

                goal_pattern:
                    goal.to_string(),

                commands,

            }
        );

    }

}
