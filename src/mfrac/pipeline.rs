use crate::mfrac::canonical::compute_canonical;
use crate::persistence::Database;
use crate::types::{CanonicalConfiguration, CellConfiguration};
use crate::utilities::bit_packing::pack_u64_u128;
use wyhash::wyhash;

pub fn process_mfrac(configuration: &CellConfiguration) {
    let canonical = compute_canonical(configuration);
    let hash = compute_hash(&canonical);

    let cc = CanonicalConfiguration {
        hash,
        configuration: canonical,
        next_hash: 0,
    };

    let db = Database::open();

    if db.contains(hash) {
        println!("Collapse!");
    }

    db.insert(&cc);
}

pub fn compute_hash(canonical: &Vec<u64>) -> u128 {
    // seeding for hash halves
    const SEED_1: u64 = 2;
    const SEED_2: u64 = 3;

    let bytes = postcard::to_allocvec(canonical).unwrap();

    // two-step u64 hash generation
    let h1 = wyhash(&bytes, SEED_1);
    let h2 = wyhash(&bytes, SEED_2);

    // final 128-bit hash
    pack_u64_u128(h1, h2)
}
