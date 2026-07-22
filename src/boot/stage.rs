#[derive(Debug, Clone, Copy)]
pub enum BootStage {

    Configuration,

    Memory,

    EventBus,

    Services,

    Runtime,

    Planner,

    Scheduler,

    CapabilityRegistry,

    ExternalAgents,

    HealthCheck,

    Ready,
}
