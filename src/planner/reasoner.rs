use crate::{
    capability::provider::Provider,
    planner::goal::Goal,
};


#[derive(Clone, Debug)]
pub struct PlanningDecision {

    pub capability: String,

    pub input: String,

    pub priority: u8,
}


pub struct Reasoner;


impl Reasoner {

    pub fn new() -> Self {
        Self
    }


    pub fn reason(
        &self,
        goal: &Goal,
        providers: &[Provider],
    ) -> Vec<PlanningDecision> {


        let mut decisions =
            Vec::new();


        let text =
            goal.description
                .to_lowercase();



        for provider in providers {


            let capability =
                provider.capability
                    .to_lowercase();



            if text.contains(&capability) {


                decisions.push(
                    PlanningDecision {

                        capability:
                            provider.capability.clone(),

                        input:
                            goal.description.clone(),

                        priority: 5,
                    }
                );
            }
        }



        // If no known capability matches,
        // create a general reasoning task.

        if decisions.is_empty() {


            decisions.push(
                PlanningDecision {

                    capability:
                        "answer".to_string(),

                    input:
                        goal.description.clone(),

                    priority: 1,
                }
            );
        }


        decisions
    }
}
