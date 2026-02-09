use crate::types::CanonicalConfiguration;
use std::path::PathBuf;

pub struct Database {
    db: sled::Db,
}

impl Database {
    pub fn open() -> Self {
        Self {
            db: sled::open(db_path()).unwrap(),
        }
    }

    pub fn get(&self, hash: u128) -> Option<CanonicalConfiguration> {
        let key = hash.to_be_bytes().to_vec();
        self.db
            .get(key)
            .unwrap()
            .map(|v| postcard::from_bytes(&v).unwrap())
    }

    pub fn insert(&self, canonical_configuration: &CanonicalConfiguration) {
        let key = canonical_configuration.hash.to_be_bytes();
        let value = postcard::to_allocvec(&canonical_configuration).unwrap();
        self.db.insert(key, value).unwrap();
    }

    pub fn contains(&self, hash: u128) -> bool {
        let key = hash.to_be_bytes().to_vec();
        self.db.contains_key(key).unwrap()
    }
}

pub fn db_path() -> PathBuf {
    PathBuf::from("./data")
}
