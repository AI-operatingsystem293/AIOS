use std::collections::HashMap;

use crate::planner::task::PlanTask;

pub struct Plan {
    pub tasks: Vec<PlanTask>,
    outputs: HashMap<u64, String>,
}

impl Plan {
    pub fn new() -> Self {
        Self {
            tasks: Vec::new(),
            outputs: HashMap::new(),
        }
    }

    pub fn add_task(&mut self, task: PlanTask) {
        self.tasks.push(task);
    }

    pub fn task(&self, id: u64) -> Option<&PlanTask> {
        self.tasks.iter().find(|t| t.id == id)
    }

    pub fn task_mut(
        &mut self,
        id: u64,
    ) -> Option<&mut PlanTask> {
        self.tasks.iter_mut().find(|t| t.id == id)
    }

    pub fn root_tasks(&self) -> Vec<&PlanTask> {
        self.tasks
            .iter()
            .filter(|t| t.dependencies.is_empty())
            .collect()
    }

    pub fn children(
        &self,
        id: u64,
    ) -> Vec<&PlanTask> {
        self.tasks
            .iter()
            .filter(|t| t.dependencies.contains(&id))
            .collect()
    }

    pub fn ready_tasks(&self) -> Vec<&PlanTask> {
        self.tasks
            .iter()
            .filter(|task| {
                task.dependencies
                    .iter()
                    .all(|dep| self.outputs.contains_key(dep))
            })
            .collect()
    }

    pub fn store_output(
        &mut self,
        task_id: u64,
        output: String,
    ) {
        self.outputs.insert(task_id, output);
    }

    pub fn output(
        &self,
        task_id: u64,
    ) -> Option<&String> {
        self.outputs.get(&task_id)
    }

    pub fn outputs(&self) -> &HashMap<u64, String> {
        &self.outputs
    }

    pub fn completed(&self) -> usize {
        self.outputs.len()
    }

    pub fn is_complete(&self) -> bool {
        self.completed() == self.tasks.len()
    }

    pub fn clear_outputs(&mut self) {
        self.outputs.clear();
    }

    pub fn list(&self) {
        println!();
        println!("========== EXECUTION PLAN ==========");

        for task in &self.tasks {

            println!("Task #{}", task.id);

            println!(" Name         : {}", task.name);

            println!(" Capability   : {}", task.capability);

            println!(" Priority     : {}", task.priority);

            println!(" Depends On   : {:?}", task.dependencies);

            match self.outputs.get(&task.id) {
                Some(output) => {
                    println!(" Output       : {}", output);
                }
                None => {
                    println!(" Output       : <pending>");
                }
            }

            println!("------------------------------------");
        }

        println!(
            "Completed {}/{} task(s)",
            self.completed(),
            self.tasks.len()
        );

        println!("====================================");
    }
}
