use crate::runner::{Statistic,FastqRecord};

pub struct GcPerRead {
    pub gc: Vec<f32>
}

impl GcPerRead{
    pub fn new()-> Self{
        GcPerRead{
            gc: Vec::new()
        }
    }
}

impl Statistic for GcPerRead {
    fn process(&mut self, record: &FastqRecord) {
        let mut gc_count = 0.0;
        let len = record.seq.len();
        for base in record.seq.iter() {
            match base {
                b'C' => gc_count += 1.0,
                b'G' => gc_count += 1.0,
                _ => gc_count += 0.0,
            }
        }
        self.gc.push(gc_count / len as f32 *100.0);
    }

    fn compute(&mut self) {
    }

    fn display(&self) {
        println!("\nGC Content per Read:");
        println!("{:<10} {:>10}", "Read #", "GC%");
        
        for (i, gc_percent) in self.gc.iter().enumerate() {
            println!("{:<10} {:>9.2}%", i + 1, gc_percent);
        }
    
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