use super::{
    event::Event,
    listener::EventListener,
};

pub struct EventBus {
    listeners: Vec<Box<dyn EventListener>>,
}

impl EventBus {
    pub fn new() -> Self {
        Self {
            listeners: Vec::new(),
        }
    }

    pub fn subscribe(
        &mut self,
        listener: Box<dyn EventListener>,
    ) {
        self.listeners.push(listener);
    }

    pub fn publish(
        &mut self,
        event: Event,
    ) {
        for listener in self.listeners.iter_mut() {
            listener.on_event(&event);
        }
    }

    // Compatibility for old code
    pub fn emit(
        &mut self,
        name: &str,
        data: &str,
    ) {
        let event = match name {
            "kernel.boot" => Event::KernelStarted,

            "kernel.stop" => Event::KernelStopped,

            "agent.loaded" => Event::AgentRegistered {
                id: data.to_string(),
            },

            "agent.started" => Event::AgentStarted {
                id: data.to_string(),
            },

            "agent.stopped" => Event::AgentStopped {
                id: data.to_string(),
            },

            "capability.registered" => Event::CapabilityRegistered {
                capability: data.to_string(),
            },

            _ => {
                println!("Unknown event: {} ({})", name, data);
                return;
            }
        };

        self.publish(event);
    }

    pub fn list(&self) {
        println!(
            "EventBus has {} listener(s).",
            self.listeners.len()
        );
    }
}
