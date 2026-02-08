use crate::types::cell_configuration::CellConfiguration;
use crate::types::cell_coord::CellCoord;
use fxhash::FxBuildHasher;

#[rustfmt::skip]
const NEIGHBOURS: [(i32, i32); 8] = [
    (-1, -1), (-1, 0), (-1, 1),
    (0, -1), /* 0, 0 */ (0, 1),
    (1, -1), (1, 0), (1, 1),
];

pub fn simulation(cconf: &CellConfiguration) -> CellConfiguration {
    let mut neighbours: hashbrown::HashMap<CellCoord, u8, FxBuildHasher> =
        hashbrown::HashMap::with_capacity_and_hasher(cconf.len() * 8, FxBuildHasher::default());

    // neighbour calculation
    for ccoord in cconf.iter() {
        for dxy in NEIGHBOURS {
            let entry = neighbours.entry(ccoord + dxy).or_insert(0);
            *entry += 1;
        }
    }

    let mut new_cconf = CellConfiguration::with_capacity(cconf.len() * 8);

    // apply survival rules
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
