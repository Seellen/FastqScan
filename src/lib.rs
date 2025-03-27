// src/lib.rs
pub mod runner;
pub mod statistics;
pub mod utils;

// Re-export important types for easier importing
pub use runner::{FastqRecord, Statistic, WorkflowRunner};
pub use statistics::{
    base_qual_pos_stat::BaseQualityPosStatistic, read_qual_stat::ReadQualityStatistic,
};
