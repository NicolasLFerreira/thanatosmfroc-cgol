use crate::types::cell_configuration::CellConfiguration;
use crate::types::cell_coord::CellCoord;
use std::collections::HashMap;

#[rustfmt::skip]
const NEIGHBOURS: [(i32, i32); 8] = [
    (-1, -1), (-1, 0), (-1, 1),
    (0, -1), /* 0, 0 */ (0, 1),
    (1, -1), (1, 0), (1, 1),
];

pub fn simulation(cconf: &CellConfiguration) -> CellConfiguration {
    let mut neighbours: HashMap<CellCoord, u8> = HashMap::new();

    // neighbour calculation
    for ccoord in cconf.iter() {
        for dxy in NEIGHBOURS {
            let entry = neighbours.entry(ccoord + dxy).or_insert(0);
            *entry += 1;
        }
    }

    let mut new_cconf = CellConfiguration::new();

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
