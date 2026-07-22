use crate::verification::{
    report::VerificationReport,
    result::VerificationResult,
};

pub struct VerificationEngine;


impl VerificationEngine {

    pub fn new() -> Self {
        Self
    }

    pub fn verify(
        &self,
        output: &str,
    ) -> VerificationResult {

        let report = self.run_checks(output);

        if report.passed {
            VerificationResult::Passed
        } else {
            VerificationResult::Failed(report.message)
        }
    }

    fn run_checks(
        &self,
        output: &str,
    ) -> VerificationReport {

        if output.trim().is_empty() {
            return VerificationReport::failed(
                "Empty output."
            );
        }

        if output.contains("No compatible agent") {
            return VerificationReport::failed(
                "Agent execution failed."
            );
        }

        if output.contains("Blocked by Policy") {
            return VerificationReport::failed(
                "Blocked by policy."
            );
        }

        VerificationReport::passed()
    }
}
