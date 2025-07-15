//! Resource Management and Optimization
//!
//! This module provides utilities for managing and optimizing resources,
//! such as caching, pooling, and lazy loading.

use std::collections::HashMap;
use std::hash::Hash;
use std::sync::{Arc, Mutex};

/// A generic cache for storing resources.
pub struct ResourceCache<K, V> {
    cache: Arc<Mutex<HashMap<K, V>>>,
}

impl<K: Eq + Hash + Clone, V: Clone> ResourceCache<K, V> {
    pub fn new() -> Self {
        Self {
            cache: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn get(&self, key: &K) -> Option<V> {
        let cache = self.cache.lock().unwrap();
        cache.get(key).cloned()
    }

    pub fn set(&self, key: K, value: V) {
        let mut cache = self.cache.lock().unwrap();
        cache.insert(key, value);
    }

    pub fn clear(&self) {
        let mut cache = self.cache.lock().unwrap();
        cache.clear();
    }
}

impl<K: Eq + Hash + Clone, V: Clone> Default for ResourceCache<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resource_cache() {
        let cache = ResourceCache::new();
        cache.set("key1".to_string(), "value1".to_string());
        assert_eq!(cache.get(&"key1".to_string()), Some("value1".to_string()));
        assert_eq!(cache.get(&"key2".to_string()), None);
        cache.clear();
        assert_eq!(cache.get(&"key1".to_string()), None);
    }
}
