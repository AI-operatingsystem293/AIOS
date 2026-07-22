#[derive(Clone, Debug)]
pub enum Event {

    KernelStarted,

    KernelStopped,

    AgentRegistered {
        id: String,
    },

    AgentStarted {
        id: String,
    },

    AgentStopped {
        id: String,
    },

    TaskCreated {
        id: u64,
    },

    TaskAssigned {
        id: u64,
        agent: String,
    },

    TaskCompleted {
        id: u64,
    },

    TaskFailed {
        id: u64,
        reason: String,
    },

    RecoveryStarted {
        task: u64,
    },

    RecoveryFinished {
        task: u64,
    },

    CapabilityRegistered {
        capability: String,
    },
}
