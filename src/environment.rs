use crate::object::Object;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Environment {
    store: HashMap<String, Object>,
}

impl Environment {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Environment {
            store: HashMap::new(),
        }
    }

    pub fn get(&mut self, name: String) -> Option<Object> {
        self.store.get(&name).cloned()
    }

    pub fn set(&mut self, name: String, val: &Object) {
        self.store.insert(name, val.clone());
    }
}
