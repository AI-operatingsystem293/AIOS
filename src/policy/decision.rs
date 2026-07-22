#[derive(Clone, Debug)]
pub struct SchedulingDecision {

    pub selected_agent: String,

    pub reason: String,

    pub score: u32,
}

impl SchedulingDecision {

    pub fn new(
        agent: String,
        reason: String,
        score: u32,
    ) -> Self {

        Self {

            selected_agent: agent,

            reason,

            score,
        }
    }
}
