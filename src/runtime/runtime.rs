use crate::{
    kernel::registry::AgentRegistry,
    runtime::{
        queue::RuntimeQueue,
        result::RuntimeResult,
        worker::Worker,
    },
    task::task::Task,
};

pub struct Runtime {
    queue: RuntimeQueue,
    workers: Vec<Worker>,
}

impl Runtime {

    pub fn new() -> Self {

        Self {
            queue: RuntimeQueue::new(),
            workers: vec![
                Worker::new(1),
            ],
        }
    }

    pub fn submit(
        &mut self,
        task: Task,
    ) {
        self.queue.push(task);
    }

    pub fn execute(
        &mut self,
        registry: &mut AgentRegistry,
    ) -> Vec<RuntimeResult> {

        let mut results = Vec::new();

        while let Some(task) =
            self.queue.pop()
        {
            let worker = &self.workers[0];

            let result =
                worker.execute(
                    registry,
                    &task,
                );

            results.push(result);
        }

        results
    }
}
