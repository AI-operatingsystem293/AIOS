use crate::agents::{
    echo::EchoAgent,
    math::MathAgent,
};

use crate::sdk::agent::Agent;

pub struct AgentLoader;

impl AgentLoader {
    pub fn load() -> Vec<Box<dyn Agent>> {
        vec![
            Box::new(EchoAgent::new()),
            Box::new(MathAgent::new()),
        ]
    }
}
