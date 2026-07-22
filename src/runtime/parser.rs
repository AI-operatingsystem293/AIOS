use std::fs;

use crate::runtime::manifest::AgentManifest;

pub struct ManifestParser;

impl ManifestParser {
    pub fn load(path: &str) -> Result<AgentManifest, String> {

        let text =
            fs::read_to_string(path)
                .map_err(|e| e.to_string())?;

        let mut id = String::new();
        let mut name = String::new();
        let mut version = String::new();
        let mut executable = String::new();
        let mut capabilities = Vec::new();

        for line in text.lines() {

            let line = line.trim();

            if line.is_empty() {
                continue;
            }

            if let Some((key, value)) =
                line.split_once('=')
            {

                let value =
                    value.trim().trim_matches('"');

                match key.trim() {

                    "id" => id = value.to_string(),

                    "name" => name = value.to_string(),

                    "version" => version = value.to_string(),

                    "entry" => {
                        executable =
                            value.to_string();
                    }

                    "capability" => {
                        capabilities.push(
                            value.to_string(),
                        );
                    }

                    _ => {}
                }
            }
        }

        let mut manifest =
            AgentManifest::new(
                &id,
                &name,
                &version,
                &executable,
            );

        for capability in capabilities {
            manifest =
                manifest.capability(&capability);
        }

        Ok(manifest)
    }
}
