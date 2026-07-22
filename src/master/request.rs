#[derive(Clone, Debug)]
pub struct MasterRequest {

    pub input: String,
}

impl MasterRequest {

    pub fn new(
        input: impl Into<String>,
    ) -> Self {

        Self {

            input: input.into(),
        }
    }
}
