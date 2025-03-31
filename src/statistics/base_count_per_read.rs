use serde::{Deserialize, Serialize};

use crate::runner::{FastqRecord, Statistic};

#[derive(Default, Serialize, Deserialize)]
pub struct BaseCountRead {
    pub gc: Vec<f32>,
}

impl BaseCountRead {
    pub fn new() -> Self {
        BaseCountRead { gc: Vec::new() }
    }

    fn _out(&self) {
        // Calculate and display summary statistics
        if !self.gc.is_empty() {
            let min = self.gc.iter().fold(f32::INFINITY, |a, &b| a.min(b));
            let max = self.gc.iter().fold(f32::NEG_INFINITY, |a, &b| a.max(b));
            let sum: f32 = self.gc.iter().sum();
            let mean = sum / self.gc.len() as f32;

            println!("\nSummary Statistics:");
            println!("{:<15} {:>8.2}%", "Minimum GC%", min);
            println!("{:<15} {:>8.2}%", "Maximum GC%", max);
            println!("{:<15} {:>8.2}%", "Average GC%", mean);
            println!("{:<15} {:>8}", "Total Reads", self.gc.len());
        }
    }
}

#[typetag::serde]
impl Statistic for BaseCountRead {
    // Processes a FastqRecord to calculate GC content.
    fn process(&mut self, record: &FastqRecord) {
        let mut gc_count = 0.0;
        let len = record.seq.len();
        // Iterate over the sequence and count G and C bases
        for base in record.seq.iter() {
            match base {
                b'C' => gc_count += 1.0,
                b'G' => gc_count += 1.0,
                _ => gc_count += 0.0,
            }
        }
        self.gc.push(gc_count / len as f32 * 100.0);
    }
}
