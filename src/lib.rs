// src/lib.rs
pub mod utils;
pub mod runner;
pub mod statistics;

// Re-export important types for easier importing
pub use runner::{WorkflowRunner, Statistic, FastqRecord};
pub use statistics::{base_qual_pos_stat::BaseQualityPosStatistic, 
                    read_qual_stat::ReadQualityStatistic};