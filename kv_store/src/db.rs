use std::collections::HashMap;

#[derive(Debug)]
pub struct Database {
    // Our mini in-memory database
    pub store: HashMap<String, String>,
}

impl Database {
    // Create a new database with an empty HashMap
    pub fn new() -> Self {
        Database {
            store: HashMap::new(),
        }
    }

    // SET key value
    pub fn set(&mut self, key: String, value: String) {
        self.store.insert(key, value);
    }

    // GET key → Option<&String>
    pub fn get(&self, key: &str) -> Option<&String> {
        self.store.get(key)
    }

    // DELETE key → bool (whether deleted or not)
    pub fn delete(&mut self, key: &str) -> bool {
        self.store.remove(key).is_some()
    }

    // LIST all entries
    pub fn list(&self) {
        for (k, v) in &self.store {
            println!("{} = {}", k, v);
        }
    }

    // EXISTS key → bool
    pub fn exists(&self, key: &str) -> bool {
        self.store.contains_key(key)
    }

    // COUNT entries
    pub fn count(&self) -> usize {
        self.store.len()
    }

    // CLEAR everything
    pub fn clear(&mut self) {
        self.store.clear();
    }

    // KEYS
    pub fn keys(&self) {
        for key in self.store.keys() {
            println!("{}", key);
        }
    }
}
