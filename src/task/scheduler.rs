use crate::{
    kernel::registry::AgentRegistry,

    recovery::{
        engine::RecoveryEngine,
        result::RecoveryResult,
    },

    runtime::{
        message::ExecutionJob,
        pool::WorkerPool,
    },

    task::{
        manager::TaskManager,
        status::TaskStatus,
    },
};


pub struct TaskScheduler {

    pool: WorkerPool,

}



impl TaskScheduler {


    pub fn new(
        pool: WorkerPool,
        ) -> Self {

    Self {

     pool,

    }

}


    pub fn tick(

        &self,

        manager: &mut TaskManager,

        registry: &mut AgentRegistry,

    ) {


        let recovery =
            RecoveryEngine::new();



        loop {


            let mut selected_tasks =
                Vec::new();



            // Find all tasks ready for parallel execution

            for (index, task) in manager.tasks.iter().enumerate() {


                if matches!(
                    task.status,
                    TaskStatus::Pending
                ) {

                    selected_tasks.push(index);

                }


            }



            if selected_tasks.is_empty() {


                println!(
                    "Scheduler: No pending tasks."
                );


                break;


            }



            println!();

            println!(
                "Parallel Scheduler: {} task(s) ready",
                selected_tasks.len()
            );




            // Submit tasks to worker pool

            for index in &selected_tasks {


                let task =
                    &mut manager.tasks[*index];



                task.status =
                    TaskStatus::Running;



                println!(
                    "Submitting Task #{}",
                    task.id
                );



                let job =
                    ExecutionJob {


                        task_id:
                            task.id,


                        command:
                            task.command.clone(),


                        input:
                            task.input.clone(),


                    };



                self.pool.submit(
                    job
                );


            }




            // Collect finished worker results

            let submitted = selected_tasks.len();

            for _ in 0..submitted {

                let result = {
                    self.pool
                        .results
                        .lock()
                        .unwrap()
                        .recv()
                        .unwrap()
                };

                println!(
                    "Worker completed Task #{}",
                    result.task_id
                );

                let task = manager
                    .tasks
                    .iter_mut()
                    .find(|t| t.id == result.task_id);

                let Some(task) = task else {
                    continue;
                };

                if result.success {

                    task.status = TaskStatus::Completed;
                    task.result = Some(result.output);

                } else {

                    task.status = TaskStatus::Failed;

                    let outcome = recovery.recover(
                        registry.providers().providers(),
                        task.assigned_agent
                            .as_deref()
                            .unwrap_or("unknown"),
                    );

                    match outcome {

                        RecoveryResult::Retry
                            | RecoveryResult::Recovered => {

                        task.retry_count += 1;

                        println!(
                            "Recovery retry attempt {}/3 for Task #{}",
                            task.retry_count,
                            task.id
                        );

                        if task.retry_count >= 3 {

                        task.status = TaskStatus::Failed;

                        task.result = Some(
                             "Maximum recovery attempts reached".to_string()
                        );

                        } else {

                        task.status = TaskStatus::Pending;
                             }
                        }

                        RecoveryResult::Replanned => {

                            task.result = Some(
                                "Needs replanning".to_string()
                            );
                        }

                        RecoveryResult::Failed => {

                            task.result = Some(
                                "Recovery failed".to_string()
                            );
                        }
                    }
                }
            }



        }


    }


}
