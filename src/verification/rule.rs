pub trait VerificationRule {
    fn verify(
        &self,
        output: &str,
    ) -> Result<(), String>;
}	
