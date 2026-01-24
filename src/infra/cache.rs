use std::collections::HashMap;
use std::sync::Mutex;

pub struct Cache {
    inner: Mutex<HashMap<String, String>>,
    _size: usize,
}

impl Cache {
    pub fn new(size: usize) -> Self {
        Self {
            inner: Mutex::new(HashMap::new()),
            _size: size,
        }
    }

    pub fn set_simple(&self, key: &str, val: String) {
        if let Ok(mut m) = self.inner.lock() {
            m.insert(key.to_string(), val);
        }
    }

    pub fn get_simple(&self, key: &str) -> Option<String> {
        if let Ok(m) = self.inner.lock() {
            m.get(key).cloned()
        } else {
            None
        }
    }
}
// End of cache implementation
