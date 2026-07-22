use crate::plugin::dynamic_loader::DynamicPluginLoader;

use crate::{
    kernel::registry::AgentRegistry,
    plugin::{
        discovery::PluginDiscovery,
        registry::PluginRegistry,
    },
    sdk::plugin::AgentPlugin,
};


pub struct PluginManager {

    loader: DynamicPluginLoader,

    registry: PluginRegistry,

}


impl PluginManager {

    pub fn new() -> Self {

        Self {

            loader: DynamicPluginLoader::new(),

            registry: PluginRegistry::new(),

        }

    }


    pub fn load_dynamic_plugins(
        &mut self,
        agent_registry: &mut AgentRegistry,
    ) {

        let discovered =
            std::fs::read_dir("plugins");


        let Ok(entries) = discovered else {
            return;
        };


        for entry in entries.flatten() {

            let path = entry.path();


            if let Some(extension) =
                path.extension()
            {

                if extension == "so" {

                    println!(
                        "Loading dynamic plugin: {:?}",
                        path
                    );


                    unsafe {

                        if let Some((library, agent)) =
                            self.loader.load(
                                path.to_str().unwrap()
                            )
                        {

                            agent_registry.keep_library(
                                library
                            );


                            agent_registry.register(
                                agent
                            );


                            println!(
                                "✓ Dynamic agent loaded"
                            );

                        }

                    }

                }

            }

        }

    }



    pub fn register_plugin(
        &mut self,
        plugin: Box<dyn AgentPlugin>,
    ) {

        self.registry.register(
            plugin
        );

    }



    pub fn discover(
        &self,
    ) {

        let scanner =
            PluginDiscovery::new();


        let plugins =
            scanner.scan(
                "plugins"
            );


        println!();

        println!(
            "========== DISCOVERED PLUGINS =========="
        );


        if plugins.is_empty() {

            println!(
                "No plugins discovered."
            );

        }


        for plugin in plugins {

            println!(
                "✓ {} v{}",
                plugin.name,
                plugin.version
            );


            println!(
                "  Author: {}",
                plugin.author
            );


            println!(
                "  {}",
                plugin.description
            );


            println!(
                "  Capabilities:"
            );


            for capability in plugin.capabilities {

                println!(
                    "    - {}",
                    capability
                );

            }


            println!();

        }

    }




    pub fn load_plugins(
        &mut self,
        agent_registry: &mut AgentRegistry,
    ) {

        println!(
            "Plugin Manager: loading plugins..."
        );


        for plugin in self.registry.plugins() {

            let agent =
                plugin.create();


            agent_registry.register(
                agent
            );

        }


        println!(
            "Plugin Manager: plugins loaded."
        );

    }



    pub fn registry(
        &self,
    ) -> &PluginRegistry {

        &self.registry

    }

}
