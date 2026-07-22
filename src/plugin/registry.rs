use crate::sdk::plugin::AgentPlugin;


pub struct PluginRegistry {

    plugins: Vec<Box<dyn AgentPlugin>>,

}


impl PluginRegistry {


    pub fn new() -> Self {

        Self {

            plugins: Vec::new(),

        }

    }



    pub fn register(
        &mut self,
        plugin: Box<dyn AgentPlugin>,
    ) {

        println!(
            "Plugin registered: {}",
            plugin.manifest().name
        );


        self.plugins.push(plugin);

    }



    pub fn plugins(
        &self,
    ) -> &Vec<Box<dyn AgentPlugin>> {

        &self.plugins

    }

}
