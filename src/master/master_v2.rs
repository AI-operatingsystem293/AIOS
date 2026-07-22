use crate::master::{
    context::MasterContext,
    state::MasterState,
};

pub struct MasterAgentV2 {
    state: MasterState,
}

impl MasterAgentV2 {
    pub fn new() -> Self {
        Self {
            state: MasterState::Idle,
        }
    }

    pub fn execute(
        &mut self,
        goal: &str,
    ) {
        let context = MasterContext::new(goal);

        self.state = MasterState::Planning;

        println!("========== MASTER ==========");
        println!("Goal: {}", context.goal);
        println!("State: {:?}", self.state);
        println!("============================");
    }

    pub fn state(&self) -> &MasterState {
        &self.state
    }
}
