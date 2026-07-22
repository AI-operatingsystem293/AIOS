 use crate::sdk::agent::Agent;
use crate::plugin::manifest::PluginManifest;


pub trait AgentPlugin {

    fn manifest(
        &self,
    ) -> PluginManifest;


    fn create(
        &self,
    ) -> Box<dyn Agent>;

}
