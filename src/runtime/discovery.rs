use std::{
    fs,
    path::Path,
};

pub struct AgentDiscovery;

impl AgentDiscovery {

    pub fn discover<P: AsRef<Path>>(
        directory: P,
    ) -> Vec<String> {

        let mut manifests = Vec::new();

        if let Ok(entries) =
            fs::read_dir(directory)
        {

            for entry in entries.flatten() {

                let path = entry.path();

                if path.is_dir() {

                    let manifest =
                        path.join("agent.toml");

                    if manifest.exists() {

                        manifests.push(
                            manifest
                                .to_string_lossy()
                                .to_string(),
                        );
                    }
                }
            }
        }

        manifests
    }
}
