use std::collections::HashMap;

use crate::memory::storage::MemoryStorage;

pub struct MemoryService {
    store: HashMap<String, String>,
}

impl MemoryService {
    pub fn new() -> Self {
        Self {
            store: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: &str, value: &str) {
        self.store
            .insert(key.to_string(), value.to_string());
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.store.get(key)
    }

    pub fn delete(&mut self, key: &str) {
        self.store.remove(key);
    }

    pub fn list(&self) {
        println!();
        println!("========== MEMORY ==========");

        if self.store.is_empty() {
            println!("(empty)");
        } else {
            for (k, v) in &self.store {
                println!("{} = {}", k, v);
            }
        }

        println!("============================");
    }

    pub fn save(&self) {
        let mut text = String::new();

        for (k, v) in &self.store {
            text.push_str(&format!("{}={}\n", k, v));
        }

        MemoryStorage::save(&text);

        println!("✓ Memory saved.");
    }

    pub fn load(&mut self) {
        let text = MemoryStorage::load();

        self.store.clear();

        for line in text.lines() {
            if let Some((k, v)) = line.split_once('=') {
                self.store
                    .insert(k.to_string(), v.to_string());
            }
        }

        println!("✓ Memory loaded.");
    }

    pub fn clear(&mut self) {
        self.store.clear();
    }

    pub fn count(&self) -> usize {
        self.store.len()
    }
}
