use crate::plugin::dynamic_loader::DynamicPluginLoader;

use crate::{
    kernel::registry::AgentRegistry,
    plugin::{discovery::PluginDiscovery, registry::PluginRegistry},
    sdk::plugin::AgentPlugin,
};

use std::collections::HashSet;
use std::path::PathBuf;

pub struct PluginManager {
    loader: DynamicPluginLoader,
    registry: PluginRegistry,
    loaded_dynamic_plugins: HashSet<PathBuf>,
}

impl PluginManager {
    pub fn new() -> Self {
        Self {
            loader: DynamicPluginLoader::new(),
            registry: PluginRegistry::new(),
            loaded_dynamic_plugins: HashSet::new(),
        }
    }

    pub fn load_dynamic_plugins(&mut self, agent_registry: &mut AgentRegistry) {
        let discovered = std::fs::read_dir("plugins");

        let Ok(entries) = discovered else {
            println!("Plugin directory not found.");
            return;
        };

        for entry in entries.flatten() {
            let path = entry.path();

            let is_plugin = matches!(
                path.extension().and_then(|e| e.to_str()),
                Some("so") | Some("dll") | Some("dylib")
            );

            if !is_plugin {
                continue;
            }

            println!("Loading dynamic plugin: {:?}", path);

            unsafe {
                if let Some((library, agent)) = self.loader.load(path.to_str().unwrap()) {
                    self.loaded_dynamic_plugins.insert(path.clone());

                    /*
                     * Keep the shared library alive for as long
                     * as the agent is registered.
                     */
                    agent_registry.keep_library(library);

                    let manifest = agent.manifest();

                    println!("Loaded Agent: {}", manifest.name);

                    println!("  Version: {}", manifest.version);

                    println!("  Author: {}", manifest.author);

                    for capability in &manifest.capabilities {
                        println!("  Capability: {}", capability);
                    }

                    agent_registry.register(agent);

                    println!("✓ Dynamic agent loaded");
                } else {
                    println!("✗ Failed to load plugin: {:?}", path);
                }
            }
        }
    }

    pub fn watch_dynamic_plugins(&mut self, agent_registry: &mut AgentRegistry) {
        let discovered = std::fs::read_dir("plugins");

        let Ok(entries) = discovered else {
            return;
        };

        for entry in entries.flatten() {
            let path = entry.path();

            let is_plugin = matches!(
                path.extension().and_then(|e| e.to_str()),
                Some("so") | Some("dll") | Some("dylib")
            );

            if !is_plugin {
                continue;
            }

            if self.loaded_dynamic_plugins.contains(&path) {
                continue;
            }

            println!();
            println!("New dynamic plugin detected: {:?}", path);

            unsafe {
                if let Some((library, agent)) = self.loader.load(path.to_str().unwrap()) {
                    agent_registry.keep_library(library);

                    let manifest = agent.manifest();

                    println!("Loaded Agent: {}", manifest.name);

                    println!("  Version: {}", manifest.version);

                    println!("  Author: {}", manifest.author);

                    for capability in &manifest.capabilities {
                        println!("  Capability: {}", capability);
                    }

                    agent_registry.register(agent);

                    self.loaded_dynamic_plugins.insert(path.clone());

                    println!("✓ New dynamic agent loaded");
                } else {
                    println!("✗ Failed to load plugin: {:?}", path);
                }
            }
        }
    }

    pub fn register_plugin(&mut self, plugin: Box<dyn AgentPlugin>) {
        self.registry.register(plugin);
    }

    pub fn discover(&self) {
        let scanner = PluginDiscovery::new();

        let plugins = scanner.scan("plugins");

        println!();
        println!("========== DISCOVERED PLUGINS ==========");

        if plugins.is_empty() {
            println!("No plugins discovered.");
        }

        for plugin in plugins {
            println!("✓ {} v{}", plugin.name, plugin.version);

            println!("  Author: {}", plugin.author);

            println!("  {}", plugin.description);

            println!("  Capabilities:");

            for capability in plugin.capabilities {
                println!("    - {}", capability);
            }

            println!();
        }
    }

    pub fn load_plugins(&mut self, agent_registry: &mut AgentRegistry) {
        println!("Plugin Manager: loading plugins...");

        self.load_dynamic_plugins(agent_registry);
    }

    pub fn registry(&self) -> &PluginRegistry {
        &self.registry
    }

    pub fn registry_mut(&mut self) -> &mut PluginRegistry {
        &mut self.registry
    }
}
