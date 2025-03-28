use crate::runner::{FastqRecord, Output, Statistic};

#[derive(Default)]
pub struct NucTable {
    tab_all: Vec<CountNucleotides>,
}

impl NucTable {
    pub fn new() -> Self {
        NucTable {
            tab_all: Vec::new(),
        }
    }

    fn ensure_length(&mut self, length: usize) {
        if self.tab_all.len() < length {
            self.tab_all.resize(length, CountNucleotides::new());
        }
    }
}

impl Output for NucTable {
    fn out(&self) {
        let mut tab_gc = Vec::new();
        tab_gc.resize(self.tab_all.len(), 0.0);

        for (iter, read) in self.tab_all.iter().enumerate() {
            read.get_percentage();
            tab_gc[iter] = self.tab_all[iter].get_gc_percentage();
        }

        println!("\nNucleotide Composition:");
        println!(
            "{:<6} {:>8} {:>8} {:>8} {:>8} {:>8} {:>8}",
            "Pos", "A%", "C%", "G%", "T%", "N%", "GC%"
        );

        for (i, counts) in self.tab_all.iter().enumerate() {
            let (a, c, g, t, n) = counts.get_percentage();
            let gc_percent = if i < tab_gc.len() {
                tab_gc[i]
            } else {
                0.0 // Default value if tab_gc hasn't been computed for this position
            };
            println!(
                "{:<6} {:>7.2}% {:>7.2}% {:>7.2}% {:>7.2}% {:>7.2}% {:>7.2}%",
                i + 1,
                a * 100.0,
                c * 100.0,
                g * 100.0,
                t * 100.0,
                n * 100.0,
                gc_percent
            );
        }
    }
}

impl Statistic for NucTable {
    fn process(&mut self, record: &FastqRecord) {
        self.ensure_length(record.seq.len());
        for (index, base) in record.seq.iter().enumerate() {
            self.tab_all[index].add_base(*base);
        }
    }
}

#[derive(Debug, Clone, Default)]
struct CountNucleotides {
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

    pub fn get_gc_percentage(&self) -> f32 {
        let total = self.a + self.c + self.g + self.t + self.n;
        if total == 0 {
            return 0.0;
        }
        ((self.g + self.c) as f32 / total as f32) * 100.0
    }
}
