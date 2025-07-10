// src/bgp/rib.rs

use std::collections::HashMap;

pub struct Rib {
    // Map prefix -> path attributes (simplified)
    routes: HashMap<String, String>,
}

impl Rib {
    pub fn new() -> Self {
        Rib {
            routes: HashMap::new(),
        }
    }

    pub fn add_route(&mut self, prefix: String, attrs: String) {
        self.routes.insert(prefix, attrs);
    }

    pub fn remove_route(&mut self, prefix: &str) {
        self.routes.remove(prefix);
    }

    pub fn dump(&self) -> Vec<(String, String)> {
        self.routes.iter().map(|(k, v)| (k.clone(), v.clone())).collect()
    }
}
