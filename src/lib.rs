// src/lib.rs
pub mod runner;
pub mod statistics;
pub mod utils;

// Re-export important types for easier importing
pub use runner::{FastqRecord, Statistic, WorkflowRunner};
pub use statistics::{phred_per_pos::PhredPerPos, phred_per_read::PhredPerRead};
