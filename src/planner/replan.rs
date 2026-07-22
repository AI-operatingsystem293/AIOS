use crate::task::task::Task;

pub struct Replanner;

impl Replanner {

    pub fn replan(
        failed: &Task,
    ) -> Vec<Task> {

        println!(
            "Replanning task {}",
            failed.id
        );

        vec![failed.clone()]
    }
}
