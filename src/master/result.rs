#[derive(Clone, Debug)]
pub enum TaskResultStatus {
    Success,
    Failed,
}

#[derive(Clone, Debug)]
pub struct TaskResult {
    pub task_id: u64,
    pub agent: String,
    pub status: TaskResultStatus,
    pub output: String,
}

impl TaskResult {
    pub fn success(
        task_id: u64,
        agent: &str,
        output: &str,
    ) -> Self {
        Self {
            task_id,
            agent: agent.to_string(),
            status: TaskResultStatus::Success,
            output: output.to_string(),
        }
    }

    pub fn failed(
        task_id: u64,
        agent: &str,
        reason: &str,
    ) -> Self {
        Self {
            task_id,
            agent: agent.to_string(),
            status: TaskResultStatus::Failed,
            output: reason.to_string(),
        }
    }
}
