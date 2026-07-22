use crate::event::{
    event::Event,
    listener::EventListener,
};

pub struct LoggerListener;

impl LoggerListener {
    pub fn new() -> Self {
        Self
    }
}

impl EventListener for LoggerListener {
    fn on_event(
        &mut self,
        event: &Event,
    ) {
        println!("[EVENT] {:?}", event);
    }
}
