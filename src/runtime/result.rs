#[derive(Debug, Clone)]
pub struct RuntimeResult {
    pub task_id: u64,
    pub success: bool,
    pub output: String,
}

impl RuntimeResult {
    pub fn success(
        task_id: u64,
        output: String,
    ) -> Self {
        Self {
            task_id,
            success: true,
            output,
        }
    }

    pub fn failed(
        task_id: u64,
        output: String,
    ) -> Self {
        Self {
            task_id,
            success: false,
            output,
        }
    }
}
