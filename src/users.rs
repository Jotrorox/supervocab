use crate::key::Key;

#[derive(Debug)]
pub struct Users {
    pub keys: Vec<Key>,
}

impl Users {
    pub fn new(keys: Vec<Key>) -> Self {
        Self { keys }
    }
    pub fn add_key(&mut self, key: Key) {
        self.keys.push(key);
    }
}