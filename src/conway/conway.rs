use crate::types::CellConfiguration;
use crate::types::CellCoord;
use fxhash::FxBuildHasher;

// Pre-computed neighbour deltas.
// Benchmarks showed decent speed gains.
#[rustfmt::skip]
const NEIGHBOURS: [(i32, i32); 8] = [
    (-1, -1), (-1, 0), (-1, 1),
    (0, -1), /* 0, 0 */ (0, 1),
    (1, -1), (1, 0), (1, 1),
];

/// Steps the simulation for a given Conway's Game of Life cell configuration
///
/// # Parameters
/// - `cconf`: Reference to the `CellConfiguration` to be stepped.
///
/// # Returns
/// A new, stepped `CellConfiguration`
pub fn step(cconf: &CellConfiguration) -> CellConfiguration {
    // 'hashbrown' implementation using FxBuildHasher.
    // Is pre-allocated with the current cconf length times 8 to avoid resizes and rehashes.
    // Benchmarks and profilers showcased huge speed gains over std::collections::HashMap.
    let mut neighbours: hashbrown::HashMap<CellCoord, u8, FxBuildHasher> =
        hashbrown::HashMap::with_capacity_and_hasher(cconf.len() * 8, FxBuildHasher::default());

    // neighbour calculation
    for ccoord in cconf.iter() {
        // iterates through the constant neighbour deltas
        for dxy in NEIGHBOURS {
            // .entry() is used to avoid extra hashes with a .contains() and .insert() approach
            let entry = neighbours.entry(ccoord + dxy).or_insert(0);
            *entry += 1;
        }
    }

    // pre-allocated with space
    let mut new_cconf = CellConfiguration::with_capacity(cconf.len() * 8);

    // apply spawn conditions
    for (coord, count) in neighbours {
        if cconf.is_alive(coord) && count == 2 {
            new_cconf.spawn(coord);
        }
        if count == 3 {
            new_cconf.spawn(coord);
        }
    }

    new_cconf
}
