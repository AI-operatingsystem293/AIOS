#[derive(Debug, Clone)]
pub struct VerificationReport {
    pub passed: bool,
    pub message: String,
}

impl VerificationReport {
    pub fn passed() -> Self {
        Self {
            passed: true,
            message: "Verification passed.".to_string(),
        }
    }

    pub fn failed(message: impl Into<String>) -> Self {
        Self {
            passed: false,
            message: message.into(),
        }
    }
}
