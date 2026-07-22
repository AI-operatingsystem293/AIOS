use crate::runtime::manifest::AgentManifest;

pub struct ManifestValidator;

impl ManifestValidator {

    pub fn validate(
        manifest: &AgentManifest,
    ) -> Result<(), Vec<String>> {

        let mut errors = Vec::new();

        // Identity
        if manifest.id.trim().is_empty() {
            errors.push("Missing agent id".to_string());
        }

        if manifest.name.trim().is_empty() {
            errors.push("Missing agent name".to_string());
        }

        if manifest.version.trim().is_empty() {
            errors.push("Missing version".to_string());
        }

        // SDK
        if manifest.sdk_version.trim().is_empty() {
            errors.push("Missing SDK version".to_string());
        }

        if manifest.requires_aios.trim().is_empty() {
            errors.push("Missing AIOS version requirement".to_string());
        }

        // Executable
        if manifest.executable.trim().is_empty() {
            errors.push("Missing executable".to_string());
        }

        // Metadata
        if manifest.author.trim().is_empty() {
            errors.push("Missing author".to_string());
        }

        if manifest.description.trim().is_empty() {
            errors.push("Missing description".to_string());
        }

        // Capabilities
        if manifest.capabilities.is_empty() {
            errors.push(
                "Agent exposes no capabilities".to_string(),
            );
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    pub fn print(
        result: &Result<(), Vec<String>>,
    ) {

        match result {

            Ok(_) => {

                println!("✓ Manifest Valid");
            }

            Err(errors) => {

                println!("Manifest Validation Failed");

                for error in errors {

                    println!(" - {}", error);
                }
            }
        }
    }
}
