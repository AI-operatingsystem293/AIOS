use crate::{
    capability::resolver::CapabilityResolver,
    kernel::registry::AgentRegistry,
    recovery::{engine::RecoveryEngine, result::RecoveryResult},
    runtime::{message::ExecutionJob, pool::WorkerPool},
    task::{manager::TaskManager, status::TaskStatus},
};

pub struct TaskScheduler {
    pool: WorkerPool,
}

impl TaskScheduler {
    pub fn new(pool: WorkerPool) -> Self {
        Self { pool }
    }

    pub fn tick(
        &self,
        manager: &mut TaskManager,
        registry: std::sync::Arc<std::sync::Mutex<AgentRegistry>>,
    ) {
        let recovery = RecoveryEngine::new();

        loop {
            let selected_tasks: Vec<usize> = manager
                .tasks
                .iter()
                .enumerate()
                .filter_map(|(index, task)| {
                    if matches!(task.status, TaskStatus::Pending) {
                        Some(index)
                    } else {
                        None
                    }
                })
                .collect();

            if selected_tasks.is_empty() {
                println!("Scheduler: No pending tasks.");
                break;
            }

            println!();
            println!("Parallel Scheduler: {} task(s) ready", selected_tasks.len());

            /*
             * Resolve ONLY the capability requested by the planner.
             *
             * Never replace an unresolved capability with an unrelated
             * provider selected from the natural-language request.
             */
            for index in &selected_tasks {
                let task = &mut manager.tasks[*index];

                let resolver = CapabilityResolver::new();

                let provider = {
                    let registry = registry.lock().unwrap();

                    resolver
                        .resolve(registry.providers(), &task.command)
                        .map(|provider| (provider.agent_id.clone(), provider.capability.clone()))
                };

                match provider {
                    Some((agent, capability)) => {
                        task.assigned_agent = Some(agent.clone());
                        task.command = capability;

                        println!("Task #{} -> Agent {}", task.id, agent);
                    }

                    None => {
                        println!(
                            "Task #{} -> no provider for capability '{}'",
                            task.id, task.command
                        );

                        task.status = TaskStatus::Failed;

                        task.result = Some(format!(
                            "No registered provider can handle capability '{}'.",
                            task.command
                        ));
                    }
                }
            }

            /*
             * Submit successfully resolved tasks.
             */
            let mut submitted = 0usize;

            for index in &selected_tasks {
                let task = &mut manager.tasks[*index];

                if !matches!(task.status, TaskStatus::Pending) {
                    continue;
                }

                task.status = TaskStatus::Running;

                println!("Submitting Task #{}", task.id);

                let job = ExecutionJob {
                    task_id: task.id,
                    command: task.command.clone(),
                    input: task.input.clone(),
                };

                self.pool.submit(job);

                submitted += 1;
            }

            /*
             * Collect worker results.
             */
            for _ in 0..submitted {
                let result = { self.pool.results.lock().unwrap().recv().unwrap() };

                println!("Worker completed Task #{}", result.task_id);

                let task = manager
                    .tasks
                    .iter_mut()
                    .find(|task| task.id == result.task_id);

                let Some(task) = task else {
                    continue;
                };

                if result.success {
                    task.status = TaskStatus::Completed;
                    task.result = Some(result.output);
                    continue;
                }

                task.status = TaskStatus::Failed;

                let outcome = {
                    let registry = registry.lock().unwrap();

                    recovery.recover(
                        registry.providers().providers(),
                        &task.command,
                        task.assigned_agent.as_deref().unwrap_or("unknown"),
                    )
                };

                match outcome {
                    RecoveryResult::Retry | RecoveryResult::Recovered => {
                        task.retry_count += 1;

                        println!(
                            "Recovery retry attempt {}/3 for Task #{}",
                            task.retry_count, task.id
                        );

                        if task.retry_count >= 3 {
                            task.status = TaskStatus::Failed;

                            task.result = Some("Maximum recovery attempts reached".to_string());
                        } else {
                            task.status = TaskStatus::Pending;
                        }
                    }

                    RecoveryResult::Replanned => {
                        task.result = Some("Needs replanning".to_string());
                    }

                    RecoveryResult::Failed => {
                        task.result = Some("Recovery failed".to_string());
                    }
                }
            }
        }
    }
}
