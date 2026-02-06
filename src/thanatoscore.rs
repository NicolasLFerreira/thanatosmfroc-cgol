use crate::Grid;
use crate::canonical::compute_canonical;

/// Heart of Thanatos: MFROC
pub fn thanatos_core(configuration: &Grid) {
    let canonical = compute_canonical(configuration);
}
