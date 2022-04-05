//! ARM Assembly module.



/// Assembly related to interrupts.
mod ints;

/// Miscellaneous assembly.
mod misc;

/// Assembly related to synchronization.
mod sync;



pub use ints::*;
pub use misc::*;
pub use sync::*;
