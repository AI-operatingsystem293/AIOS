use crate::runtime::{
    manifest::AgentManifest,
    message::{
        MessageType,
        RuntimeMessage,
    },
};

pub struct Handshake;

impl Handshake {

    pub fn hello() -> RuntimeMessage {

        RuntimeMessage::new(
            1,
            0,
            MessageType::Heartbeat,
            "system",
            "HELLO",
        )
    }

    pub fn request_manifest() -> RuntimeMessage {

        RuntimeMessage::new(
            2,
            0,
            MessageType::Execute,
            "system",
            "REQUEST_MANIFEST",
        )
    }

    pub fn ready() -> RuntimeMessage {

        RuntimeMessage::new(
            3,
            0,
            MessageType::Heartbeat,
            "system",
            "READY",
        )
    }

    pub fn validate(
        manifest: &AgentManifest,
    ) -> Result<(), String> {

        if manifest.id.is_empty() {
            return Err("Missing id".into());
        }

        if manifest.name.is_empty() {
            return Err("Missing name".into());
        }

        if manifest.version.is_empty() {
            return Err("Missing version".into());
        }

        if manifest.executable.is_empty() {
            return Err("Missing executable".into());
        }

        if manifest.capabilities.is_empty() {
            return Err(
                "Agent exposes no capabilities".into(),
            );
        }

        Ok(())
    }
}
