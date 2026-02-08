//! The `conway` module contains the Conway's Game of Life implementation.
//! It provides a single public API `step` for stepping Conway configurations to their next iteration.

mod conway;
pub use conway::step;
