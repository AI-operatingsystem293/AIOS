#[derive(Debug, Clone)]
pub enum VerificationResult {
    Passed,
    Failed(String),
}
