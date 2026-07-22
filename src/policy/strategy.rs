#[derive(Clone, Debug)]
pub enum SchedulingStrategy {

    LowestLatency,

    HighestReliability,

    LowestLoad,

    RoundRobin,

    HighestScore,
}
