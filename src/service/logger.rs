use crate::service::service::Service;

pub struct LoggerService {
    running: bool,
}

impl LoggerService {
    pub fn new() -> Self {
        Self {
            running: false,
        }
    }
}

impl Service for LoggerService {
    fn name(&self) -> &'static str {
        "Logger"
    }

    fn start(&mut self) {
        self.running = true;
    }

    fn stop(&mut self) {
        self.running = false;
    }

    fn status(&self) -> &'static str {
        if self.running {
            "Running"
        } else {
            "Stopped"
        }
    }
}
