use super::status::TaskStatus;

#[derive(Debug, Clone)]
pub struct Task {
    pub id: u64,

    // Parent task (None = root task)
    pub parent_id: Option<u64>,

    // Command to execute
    pub command: String,

    // User input
    pub input: String,

    // Agent assigned to execute
    pub assigned_agent: Option<String>,

    // Task priority
    pub priority: u8,

    // Current state
    pub status: TaskStatus,

    // Result after execution
    pub result: Option<String>,

    // Future timestamps
    pub created_at: u64,
    pub completed_at: Option<u64>,

    // Recovery retry counter
    pub retry_count: u32,
}

impl Task {
    pub fn new(
        id: u64,
        parent_id: Option<u64>,
        command: String,
        input: String,
        priority: u8,
    ) -> Self {
        Self {
            id,
            parent_id,
            command,
            input,
            assigned_agent: None,
            priority,
            status: TaskStatus::Pending,
            result: None,
            created_at: 0,
            completed_at: None,
            retry_count: 0,
        }
    }
}
