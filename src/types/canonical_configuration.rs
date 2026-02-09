use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CanonicalConfiguration {
    pub hash: u128,
    pub configuration: Vec<u64>,
    pub next_hash: u128,
}
