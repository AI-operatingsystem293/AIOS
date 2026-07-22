pub struct PolicyEngine;

impl PolicyEngine {
    pub fn new() -> Self {
        Self
    }

    pub fn allow(&self, command: &str) -> bool {
        match command {

            // Dangerous commands
            "delete" => false,
            "format" => false,
            "shutdown" => false,

            // Everything else
            _ => true,
        }
    }
}
