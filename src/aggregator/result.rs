#[derive(Clone, Debug)]
pub struct TaskResult {

    pub task_id: u64,

    pub agent: String,

    pub output: String,

    pub success: bool,
}

impl TaskResult {

    pub fn new(
        task_id: u64,
        agent: String,
        output: String,
        success: bool,
    ) -> Self {

        Self {

            task_id,

            agent,

            output,

            success,
        }
    }
}
