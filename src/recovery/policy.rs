#[derive(Clone, Debug)]
pub enum RecoveryPolicy {

    RetrySameAgent,

    TryAnotherAgent,

    Replan,

    Abort,
}
