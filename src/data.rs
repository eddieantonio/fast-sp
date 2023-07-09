//! Random data for tests and benchmarks.

/// Random ASCII 's' and 'p' bytes.
pub const RANDOM_SP: &str = include_str!(concat!(env!("OUT_DIR"), "/random-sp.bin"));
/// Random ASCII printable characters.
pub const RANDOM_PRINTABLE: &str = include_str!(concat!(env!("OUT_DIR"), "/random-printable.bin"));
