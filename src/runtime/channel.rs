use std::collections::VecDeque;

use crate::runtime::message::RuntimeMessage;

pub struct AgentChannel {
    inbox: VecDeque<RuntimeMessage>,
    outbox: VecDeque<RuntimeMessage>,
}

impl AgentChannel {

    pub fn new() -> Self {
        Self {
            inbox: VecDeque::new(),
            outbox: VecDeque::new(),
        }
    }

    pub fn send(
        &mut self,
        message: RuntimeMessage,
    ) {
        self.outbox.push_back(message);
    }

    pub fn receive(
        &mut self,
        message: RuntimeMessage,
    ) {
        self.inbox.push_back(message);
    }

    pub fn next_outgoing(
        &mut self,
    ) -> Option<RuntimeMessage> {
        self.outbox.pop_front()
    }

    pub fn next_incoming(
        &mut self,
    ) -> Option<RuntimeMessage> {
        self.inbox.pop_front()
    }

    pub fn pending_outgoing(
        &self,
    ) -> usize {
        self.outbox.len()
    }

    pub fn pending_incoming(
        &self,
    ) -> usize {
        self.inbox.len()
    }
}
