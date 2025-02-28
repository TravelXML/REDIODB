// src/storage/ttl_store.rs

use std::collections::{HashMap, HashSet};
use std::time::{Duration, Instant};

/// Represents the different types of values our store can hold.
#[derive(Debug, Clone)]
pub enum StoreValue {
    /// A simple string value.
    Simple(String),
    /// A list of string values.
    List(Vec<String>),
    /// A set of unique string values.
    Set(HashSet<String>),
    /// A hash mapping field names to string values.
    Hash(HashMap<String, String>),
}

/// TTLStore is an in-memory key–value store that supports TTLs and multiple data types.
pub struct TTLStore {
    store: HashMap<String, (StoreValue, Option<Instant>)>,
}

impl TTLStore {
    /// Creates a new, empty TTLStore.
    pub fn new() -> Self {
        TTLStore {
            store: HashMap::new(),
        }
    }

    /// Helper method: Check if the key has expired.
    /// If expired, remove it from the store.
    fn check_expiry(&mut self, key: &str) {
        if let Some((_, Some(expiry))) = self.store.get(key) {
            if Instant::now() >= *expiry {
                self.store.remove(key);
            }
        }
    }

    /// Set a key with a simple string value and optional TTL.
    pub fn set(&mut self, key: &str, value: &str, ttl: Option<Duration>) {
        let expiry = ttl.map(|dur| Instant::now() + dur);
        self.store.insert(
            key.to_string(),
            (StoreValue::Simple(value.to_string()), expiry),
        );
    }

    /// Get the value for a key (if it exists and is a Simple value).
    pub fn get(&mut self, key: &str) -> Option<String> {
        self.check_expiry(key);
        if let Some((StoreValue::Simple(ref val), _)) = self.store.get(key) {
            Some(val.clone())
        } else {
            None
        }
    }

    /// Set the expiration (TTL) for a key.
    pub fn expire(&mut self, key: &str, ttl: Duration) -> bool {
        self.check_expiry(key);
        if let Some(entry) = self.store.get_mut(key) {
            entry.1 = Some(Instant::now() + ttl);
            true
        } else {
            false
        }
    }

    /// Return the remaining TTL in seconds, or None if the key does not exist.
    /// Returns -1 if the key exists but has no TTL.
    pub fn ttl(&mut self, key: &str) -> Option<i64> {
        self.check_expiry(key);
        if let Some((_, Some(expiry))) = self.store.get(key) {
            let now = Instant::now();
            if *expiry > now {
                Some(expiry.duration_since(now).as_secs() as i64)
            } else {
                Some(0)
            }
        } else if self.store.contains_key(key) {
            Some(-1)
        } else {
            None
        }
    }

    /// Delete a key from the store.
    pub fn del(&mut self, key: &str) -> bool {
        self.store.remove(key).is_some()
    }

    /// Atomically increment a key's numeric value.
    /// If the key doesn't exist, it is created with the increment value.
    pub fn incr(&mut self, key: &str, amount: i32) -> Option<String> {
        self.check_expiry(key);
        let new_val = if let Some((StoreValue::Simple(ref mut val), _)) = self.store.get_mut(key) {
            match val.parse::<i32>() {
                Ok(num) => {
                    let updated = num + amount;
                    *val = updated.to_string();
                    updated.to_string()
                }
                Err(_) => return None,
            }
        } else {
            self.store.insert(
                key.to_string(),
                (StoreValue::Simple(amount.to_string()), None),
            );
            amount.to_string()
        };
        Some(new_val)
    }

    /// Atomically decrement a key's numeric value.
    pub fn decr(&mut self, key: &str, amount: i32) -> Option<String> {
        self.incr(key, -amount)
    }

    /// Append a string to the current value of a key.
    pub fn append(&mut self, key: &str, value: &str) -> Option<String> {
        self.check_expiry(key);
        if let Some((StoreValue::Simple(ref mut val), _)) = self.store.get_mut(key) {
            val.push_str(value);
            Some(val.clone())
        } else {
            None
        }
    }

    /// Return a list of keys matching a given pattern.
    /// For simplicity, this supports "*" for all keys or substring matching.
    pub fn keys(&mut self, pattern: &str) -> Vec<String> {
        // First, collect all keys to avoid mutable borrowing while iterating.
        let all_keys: Vec<String> = self.store.keys().cloned().collect();
        let mut result = Vec::new();
        for key in all_keys {
            self.check_expiry(&key);
            if pattern == "*" || key.contains(pattern) {
                result.push(key);
            }
        }
        result
    }

    /// List operations: push a value onto the front of the list.
    pub fn l_push(&mut self, key: &str, value: &str) {
        self.check_expiry(key);
        let entry = self
            .store
            .entry(key.to_string())
            .or_insert((StoreValue::List(Vec::new()), None));
        match &mut entry.0 {
            StoreValue::List(list) => list.insert(0, value.to_string()),
            _ => entry.0 = StoreValue::List(vec![value.to_string()]),
        }
    }

    /// List operations: pop a value from the front of the list.
    pub fn l_pop(&mut self, key: &str) -> Option<String> {
        self.check_expiry(key);
        if let Some((StoreValue::List(list), _)) = self.store.get_mut(key) {
            if !list.is_empty() {
                Some(list.remove(0))
            } else {
                None
            }
        } else {
            None
        }
    }

    /// Set operations: add a member to a set.
    pub fn s_add(&mut self, key: &str, member: &str) {
        self.check_expiry(key);
        let entry = self
            .store
            .entry(key.to_string())
            .or_insert((StoreValue::Set(HashSet::new()), None));
        match &mut entry.0 {
            StoreValue::Set(set) => {
                set.insert(member.to_string());
            }
            _ => entry.0 = StoreValue::Set([member.to_string()].iter().cloned().collect()),
        }
    }

    /// Set operations: get all members of a set.
    pub fn s_members(&mut self, key: &str) -> Vec<String> {
        self.check_expiry(key);
        if let Some((StoreValue::Set(set), _)) = self.store.get(key) {
            set.iter().cloned().collect()
        } else {
            Vec::new()
        }
    }

    /// Hash operations: set a field in a hash.
    pub fn h_set(&mut self, key: &str, field: &str, value: &str) {
        self.check_expiry(key);
        let entry = self
            .store
            .entry(key.to_string())
            .or_insert((StoreValue::Hash(HashMap::new()), None));
        match &mut entry.0 {
            StoreValue::Hash(map) => {
                map.insert(field.to_string(), value.to_string());
            }
            _ => {
                let mut map = HashMap::new();
                map.insert(field.to_string(), value.to_string());
                entry.0 = StoreValue::Hash(map);
            }
        }
    }

    /// Hash operations: get a field from a hash.
    pub fn h_get(&mut self, key: &str, field: &str) -> Option<String> {
        self.check_expiry(key);
        if let Some((StoreValue::Hash(map), _)) = self.store.get(key) {
            map.get(field).cloned()
        } else {
            None
        }
    }
}
