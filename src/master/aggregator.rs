use crate::master::result::{
    TaskResult,
    TaskResultStatus,
};

pub struct ResultAggregator {
    results: Vec<TaskResult>,
}

impl ResultAggregator {
    pub fn new() -> Self {
        Self {
            results: Vec::new(),
        }
    }

    pub fn add(
        &mut self,
        result: TaskResult,
    ) {
        self.results.push(result);
    }

    pub fn success_count(&self) -> usize {
        self.results
            .iter()
            .filter(|r| matches!(r.status, TaskResultStatus::Success))
            .count()
    }

    pub fn failed_count(&self) -> usize {
        self.results
            .iter()
            .filter(|r| matches!(r.status, TaskResultStatus::Failed))
            .count()
    }

    pub fn report(&self) {

        println!();
        println!("====== EXECUTION REPORT ======");

        for result in &self.results {

            println!(
                "#{} [{}] {}",
                result.task_id,
                result.agent,
                result.output
            );
        }

        println!();
        println!("Successful : {}", self.success_count());
        println!("Failed     : {}", self.failed_count());

        println!("==============================");
    }

    pub fn all(&self) -> &[TaskResult] {
        &self.results
    }
}
