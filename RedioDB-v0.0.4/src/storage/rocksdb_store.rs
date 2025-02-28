// src/storage/rocksdb_store.rs
//
// A simple wrapper around RocksDB for persistent key–value storage.
use rocksdb::{Options, DB};

pub struct RocksDBStore {
    db: DB,
}

impl RocksDBStore {
    /// Opens a RocksDB database at the specified path.
    pub fn new(path: &str) -> Self {
        let mut opts = Options::default();
        opts.create_if_missing(true);
        let db = DB::open(&opts, path).expect("Failed to open RocksDB");
        RocksDBStore { db }
    }

    /// Sets a key-value pair in the database.
    pub fn set(&self, key: &str, value: &str) {
        self.db.put(key.as_bytes(), value.as_bytes())
            .expect("Failed to write to RocksDB");
    }

    /// Retrieves a value for a key from the database.
    pub fn get(&self, key: &str) -> Option<String> {
        match self.db.get(key.as_bytes()) {
            Ok(Some(value)) => Some(String::from_utf8(value).unwrap_or_default()),
            _ => None,
        }
    }
}
