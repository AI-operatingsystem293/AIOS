use std::thread;

use crate::task::task::Task;


pub struct ParallelExecutor;


impl ParallelExecutor {

    pub fn new() -> Self {
        Self
    }


    pub fn execute_parallel(
        &self,
        tasks: Vec<Task>,
    ) {

        println!(
            "Parallel Executor: running {} task(s)",
            tasks.len()
        );


        let mut handles = Vec::new();


        for task in tasks {

            let handle =
                thread::spawn(
                    move || {

                        println!(
                           "Running Task #{} -> {}",
                            task.id,
                            task.command
                        );


                        // Temporary execution hook
                        // Real provider execution will connect here

                        println!(
                            "Task #{} completed",
                            task.id
                        );

                    }
                );


            handles.push(handle);

        }



        for handle in handles {

            let _ =
                handle.join();

        }


        println!(
            "Parallel execution batch completed."
        );

    }

}
