use crate::runner::{Statistic,FastqRecord};


pub struct NucTable {
    table: Vec<CountNucleotides>,
}


impl Statistic for NucTable {
    fn process(&mut self, record: &FastqRecord) {
        for (index, base) in record.seq.iter().enumerate() {
            self.table[index].add_base(*base);
        }
    }

    fn compute(&mut self) {
        for read in self.table.iter() {
            read.get_percentage();
        }
    }

    fn display(&self) {}
}


#[derive(Debug, Clone)]
pub struct CountNucleotides {
    a: u64,
    c: u64,
    g: u64,
    t: u64,
    n: u64,
}

impl CountNucleotides {
    pub fn new() -> Self {
        CountNucleotides {
            a: 0,
            c: 0,
            g: 0,
            t: 0,
            n: 0,
        }
    }

    pub fn add_base(&mut self, base: u8) {
        match base {
            b'A' => self.a += 1,
            b'C' => self.c += 1,
            b'G' => self.g += 1,
            b'T' => self.t += 1,
            _ => self.n += 1,
        }
    }

    pub fn get_percentage(&self) -> (f64, f64, f64, f64, f64) {
        let total = self.a + self.c + self.g + self.t + self.n;
        if total == 0 {
            return (0.0, 0.0, 0.0, 0.0, 0.0);
        }
        (
            self.a as f64 / total as f64,
            self.c as f64 / total as f64,
            self.g as f64 / total as f64,
            self.t as f64 / total as f64,
            self.n as f64 / total as f64,
        )
    }

    pub fn get_number(&self) -> (u64, u64, u64, u64, u64) {
        (self.a, self.c, self.g, self.t, self.n)
    }
}