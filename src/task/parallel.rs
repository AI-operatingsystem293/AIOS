use std::sync::{Arc, Mutex};
use std::thread;

use crate::{
    kernel::registry::AgentRegistry,
    task::{
        manager::TaskManager,
        status::TaskStatus,
    },
};

pub struct ParallelScheduler;

impl ParallelScheduler {
    pub fn new() -> Self {
        Self
    }

    pub fn execute(
        &self,
        manager: Arc<Mutex<TaskManager>>,
        registry: Arc<Mutex<AgentRegistry>>,
    ) {
        loop {
            let ready = {
                let mgr = manager.lock().unwrap();

                mgr.tasks
                    .iter()
                    .filter(|t| matches!(t.status, TaskStatus::Pending))
                    .map(|t| t.id)
                    .collect::<Vec<u64>>()
            };

            if ready.is_empty() {
                break;
            }

            let mut workers = Vec::new();

            for id in ready {

                let mgr = Arc::clone(&manager);
                let reg = Arc::clone(&registry);

                workers.push(thread::spawn(move || {

                    let (command, input) = {
                        let mut manager =
                            mgr.lock().unwrap();

                        let task = manager
                            .tasks
                            .iter_mut()
                            .find(|t| t.id == id)
                            .unwrap();

                        task.status = TaskStatus::Running;

                        (
                            task.command.clone(),
                            task.input.clone(),
                        )
                    };

                    let result = {
                        let mut registry =
                            reg.lock().unwrap();

                        registry.execute_command(
                            &command,
                            &input,
                        )
                    };

                    let mut manager =
                        mgr.lock().unwrap();

                    let task = manager
                        .tasks
                        .iter_mut()
                        .find(|t| t.id == id)
                        .unwrap();

                    match result {
                        Some(output) => {
                            task.status =
                                TaskStatus::Completed;

                            task.result =
                                Some(output);
                        }

                        None => {
                            task.status =
                                TaskStatus::Failed;

                            task.result = Some(
                                "Execution failed"
                                    .to_string(),
                            );
                        }
                    }
                }));
            }

            for worker in workers {
                worker.join().unwrap();
            }
        }
    }
}
