use crate::planner::goal::Goal;

pub struct GoalAnalyzer;

impl GoalAnalyzer {
    pub fn new() -> Self {
        Self
    }

    pub fn analyze(&self, text: &str) -> Goal {
        Goal::new(1, text)
    }
}
