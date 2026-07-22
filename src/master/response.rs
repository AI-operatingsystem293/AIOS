#[derive(Clone, Debug)]
pub struct MasterResponse {

    pub success: bool,

    pub output: String,
}

impl MasterResponse {

    pub fn ok(
        output: impl Into<String>,
    ) -> Self {

        Self {

            success: true,

            output: output.into(),
        }
    }

    pub fn error(
        output: impl Into<String>,
    ) -> Self {

        Self {

            success: false,

            output: output.into(),
        }
    }
}
