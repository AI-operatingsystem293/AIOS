#[derive(Clone, Debug)]
pub struct RecoveryEvent {

    pub task_id: u64,

    pub failed_agent: String,

    pub reason: String,
}
