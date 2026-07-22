use super::stage::BootStage;

pub struct BootManager {

    stage: BootStage,
}

impl BootManager {

    pub fn new() -> Self {

        Self {

            stage: BootStage::Configuration,
        }
    }

    pub fn enter(
        &mut self,
        stage: BootStage,
    ) {

        self.stage = stage;

        println!(
            "[BOOT] {:?}",
            self.stage
        );
    }

    pub fn current(
        &self,
    ) -> BootStage {

        self.stage
    }
}
