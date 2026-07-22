use crate::plugin::plugin::Plugin;

pub struct PluginLoader {
    plugins: Vec<Plugin>,
}

impl PluginLoader {
    pub fn new() -> Self {
        Self {
            plugins: Vec::new(),
        }
    }

    pub fn load(&mut self, name: &str, version: &str) {
        self.plugins.push(Plugin::new(name, version));

        println!("✓ Loaded plugin: {} v{}", name, version);
    }

    pub fn list(&self) {
        println!();
        println!("========== PLUGINS ==========");

        if self.plugins.is_empty() {
            println!("No plugins loaded.");
        } else {
            for plugin in &self.plugins {
                println!("{} v{}", plugin.name, plugin.version);
            }
        }

        println!("=============================");
    }

    pub fn count(&self) -> usize {
        self.plugins.len()
    }
}
