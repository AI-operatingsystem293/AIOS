use crate::runtime::{
    channel::AgentChannel,
    message::RuntimeMessage,
};

pub struct TaskDispatcher {
    channel: AgentChannel,
}

impl TaskDispatcher {

    pub fn new() -> Self {
        Self {
            channel: AgentChannel::new(),
        }
    }

    pub fn dispatch(
        &mut self,
        message: RuntimeMessage,
    ) {

        println!(
            "Dispatching Task {}",
            message.task_id
        );

        self.channel.send(message);
    }

    pub fn receive(
        &mut self,
        message: RuntimeMessage,
    ) {

        println!(
            "Received Result {}",
            message.task_id
        );

        self.channel.receive(message);
    }

    pub fn next_task(
        &mut self,
    ) -> Option<RuntimeMessage> {

        self.channel.next_outgoing()
    }

    pub fn next_result(
        &mut self,
    ) -> Option<RuntimeMessage> {

        self.channel.next_incoming()
    }
}
