use super::event::Event;

pub trait EventListener {

    fn on_event(
        &mut self,
        event: &Event,
    );
}
