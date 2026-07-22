use std::collections::HashMap;

use crate::runtime::{
    manifest::AgentManifest,
    process::AgentProcess,
};

pub struct AgentLauncher {
    running: HashMap<String, AgentProcess>,
}

impl AgentLauncher {

    pub fn new() -> Self {

        Self {
            running: HashMap::new(),
        }
    }

    pub fn launch(
        &mut self,
        manifest: &AgentManifest,
    ) -> Result<(), String> {

        println!(
            "Launching {}...",
            manifest.name
        );

        let process =
            AgentProcess::spawn(
                &manifest.executable,
            )?;

        println!(
            "PID {}",
            process.id()
        );

        self.running.insert(
            manifest.id.clone(),
            process,
        );

        Ok(())
    }

    pub fn running(
        &self,
    ) -> usize {

        self.running.len()
    }

    pub fn stop_all(
        &mut self,
    ) {

        for (_, process) in
            self.running.iter_mut()
        {
            let _ = process.stop();
        }

        self.running.clear();
    }
}
