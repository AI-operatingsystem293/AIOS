use std::fs::{read_to_string, write};

pub struct MemoryStorage;

impl MemoryStorage {
    pub fn save(data: &str) {
        let _ = write("memory.db", data);
    }

    pub fn load() -> String {
        read_to_string("memory.db")
            .unwrap_or_default()
    }
}
