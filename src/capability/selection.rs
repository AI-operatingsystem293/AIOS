#[derive(Clone, Debug)]
pub struct SelectedProvider {

    pub capability: String,

    pub agent: String,

    pub score: u32,
}

impl SelectedProvider {

    pub fn new(
        capability: String,
        agent: String,
        score: u32,
    ) -> Self {

        Self {

            capability,

            agent,

            score,
        }
    }
}
