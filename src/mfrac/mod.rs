//! The `mfrac` module contains the Memoized Forward-Reachability Orbit-Collapsing algorithm implementation.
//! It provides a single public API `process_mfrac` for processing through its internal pipeline.

mod canonical;
mod pipeline;
pub use pipeline::process_mfrac;
