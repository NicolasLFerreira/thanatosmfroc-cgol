use crate::types::cell_coord::CellCoord;
use fxhash::FxBuildHasher;
use rand::prelude::StdRng;
use rand::{RngExt, SeedableRng};

type InternalCells = hashbrown::HashSet<CellCoord, FxBuildHasher>;

#[derive(Default, PartialEq, Eq)]
pub struct CellConfiguration {
    internal_cells: InternalCells,
}

// Instantiation
impl CellConfiguration {
    pub fn new() -> Self {
        Self {
            internal_cells: InternalCells::default(),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            internal_cells: InternalCells::with_capacity_and_hasher(
                capacity,
                FxBuildHasher::default(),
            ),
        }
    }

    pub fn with_seed_configuration(seed_cells: Vec<CellCoord>) -> Self {
        Self {
            internal_cells: seed_cells.into_iter().collect(),
        }
    }
}

// Interaction
impl CellConfiguration {
    pub fn is_alive(&self, coord: CellCoord) -> bool {
        self.internal_cells.contains(&coord)
    }

    pub fn spawn(&mut self, coord: CellCoord) {
        self.internal_cells.insert(coord);
    }

    pub fn despawn(&mut self, coord: CellCoord) {
        self.internal_cells.remove(&coord);
    }

    pub fn iter(&self) -> impl Iterator<Item = CellCoord> {
        self.internal_cells.iter().copied()
    }

    pub fn len(&self) -> usize {
        self.internal_cells.len()
    }
}

// Utility
impl CellConfiguration {
    pub fn random_configuration(
        seed: u64,
        width: usize,
        height: usize,
        density: f64,
    ) -> Vec<CellCoord> {
        let mut rng = StdRng::seed_from_u64(seed);
        let mut cells = Vec::new();

        for y in 0..height {
            for x in 0..width {
                if rng.random::<f64>() < density {
                    cells.push(CellCoord::new(x as i32, y as i32));
                }
            }
        }

        cells
    }
}

// Clone
impl Clone for CellConfiguration {
    fn clone(&self) -> Self {
        Self {
            internal_cells: self.internal_cells.clone(),
        }
    }
}
