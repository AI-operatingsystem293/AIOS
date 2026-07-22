#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MasterState {

    Idle,

    Planning,

    Scheduling,

    Executing,

    Recovering,

    Verifying,

    Completed,

    Failed,
}
