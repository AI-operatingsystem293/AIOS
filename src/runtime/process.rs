use std::process::{
    Child,
    Command,
};

pub struct AgentProcess {
    child: Child,
}

impl AgentProcess {
    pub fn spawn(
        executable: &str,
    ) -> Result<Self, String> {

        let child = Command::new(executable)
            .spawn()
            .map_err(|e| e.to_string())?;

        Ok(Self { child })
    }

    pub fn id(&self) -> u32 {
        self.child.id()
    }

    pub fn is_running(
        &mut self,
    ) -> bool {

        self.child
            .try_wait()
            .ok()
            .flatten()
            .is_none()
    }

    pub fn stop(
        &mut self,
    ) -> Result<(), String> {

        self.child
            .kill()
            .map_err(|e| e.to_string())
    }
}
