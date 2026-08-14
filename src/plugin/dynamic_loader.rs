use libloading::{Library, Symbol};

use crate::sdk::agent::Agent;

pub struct DynamicPluginLoader;

impl DynamicPluginLoader {
    pub fn new() -> Self {
        Self
    }

    pub unsafe fn load(&mut self, path: &str) -> Option<(Library, Box<dyn Agent>)> {
        let library = Library::new(path).ok()?;

        let constructor: Symbol<unsafe extern "C" fn() -> *mut dyn Agent> =
            library.get(b"create_plugin").ok()?;

        let boxed_raw = constructor();

        if boxed_raw.is_null() {
            return None;
        }

        let agent = Box::from_raw(boxed_raw);

        Some((library, agent))
    }
}
