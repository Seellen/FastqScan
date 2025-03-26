use gnuplot::AxesCommon;
use gnuplot::Figure;
use std::io::{self, BufRead};

use crate::utils::calculate_phred;


#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct FastqRecord {
    seq: Vec<u8>,
    qual: Vec<u8>,
}

pub trait Statistic {
    /* Statistics:

    * average base quality (Phred)
    * average quality of all reads
    * average proportions of `{A, C, G, T, N}` for each read position
    * ...
    */

    fn process(&mut self, record: &FastqRecord);

    fn compute(&mut self);

    fn display(&self);
    // TODO - find a way to represent the results.
    // Let's try to identify the shared parts of *any* statistic
    // and report these in some fashion.
    // fn report(self) -> ?
}

/// Computes mean base quality for a position read.
pub struct BaseQualityPosStatistic {
    qual_sums: Vec<f32>,
    amounts: Vec<u64>,
    qual_avg: Vec<f32>,
}

impl BaseQualityPosStatistic {
    pub fn new() -> Self {
        BaseQualityPosStatistic {
            qual_sums: Vec::new(),
            amounts: Vec::new(),
            qual_avg: Vec::new(),
        }
    }
}

impl Statistic for BaseQualityPosStatistic {
    fn process(&mut self, record: &FastqRecord) {
        // Ensure vectors are large enough
        if self.qual_sums.len() < record.qual.len() {
            self.qual_sums.resize(record.qual.len(), 0.0);
            self.amounts.resize(record.qual.len(), 0);
        }

        // Convert ASCII qualities to Phred scores and sum
        for (i, &qual) in record.qual.iter().enumerate() {
            if let Some(phred) = calculate_phred(qual as u8) {
                self.qual_sums[i] += phred;
                self.amounts[i] += 1;
            } // Convert ASCII to Phred
        }
    }

    fn compute(&mut self) {
        self.qual_avg = self
            .qual_sums
            .iter()
            .zip(&self.amounts)
            .map(|(&sum, &amount)| sum / amount as f32)
            .collect()
    }

    fn display(&self) {
        let positions: Vec<usize> = (0..self.qual_avg.len()).collect(); // X-axis: positions
        let qual_values = &self.qual_avg; // Y-axis: quality scores

        let mut fg = Figure::new();
        fg.axes2d()
            .set_title("Base Quality per Position", &[])
            .set_x_label("Position", &[])
            .set_y_label("Average Quality Score", &[])
            .lines(
                positions,
                qual_values,
                &[gnuplot::Caption("Avg Quality"), gnuplot::Color("blue")],
            );

        fg.show().unwrap(); // Display the plot
    }
}

/// Computes mean base quality for a read.
pub struct ReadQualityStatistic {
    mean: Vec<f32>,
}

impl ReadQualityStatistic {
    pub fn new() -> Self {
        ReadQualityStatistic { mean: Vec::new() }
    }
}

impl Statistic for ReadQualityStatistic {
    fn process(&mut self, record: &FastqRecord) {
        // Convert quality scores and store in x
        let x: Vec<f32> = record
            .qual
            .iter()
            .filter_map(|&q| calculate_phred(q)) // Convert ASCII to Phred score
            .collect();

        // push the mean of x
        if !x.is_empty() {
            self.mean.push(x.iter().sum::<f32>() / x.len() as f32)
        } else {
            ()
        };
    }

    fn compute(&mut self) {}

    fn display(&self) {
        let read_nr: Vec<usize> = (0..self.mean.len()).collect(); // X-axis: positions
        let qual_values = &self.mean; // Y-axis: quality scores

        let mut fg = Figure::new();
        fg.axes2d()
            .set_title("Avg Quality per Read", &[])
            .set_x_label("Position", &[])
            .set_y_label("Average Quality Score", &[])
            .lines(
                qual_values,
                read_nr,
                &[gnuplot::Caption("Avg Quality"), gnuplot::Color("blue")],
            );

        fg.show().unwrap(); // Display the plot
    }
}

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

pub struct WorkflowRunner {
    pub statistics: Vec<Box<dyn Statistic>>,
}

impl WorkflowRunner {
    /// Process the FASTQ file.
    ///
    /// Can return an I/O error or other errors (not in the signature at this point)
    pub fn process<R>(&mut self, mut read: R)
    where
        R: BufRead,
    {
        let mut record = FastqRecord::default();

        while let Ok(()) = WorkflowRunner::parse_record(&mut read, &mut record) {
            for statistic in self.statistics.iter_mut() {
                statistic.process(&record);
            }
        }

        for statistic in self.statistics.iter_mut() {
            statistic.compute();
        }

        for statistic in self.statistics.iter() {
            statistic.display();
        }
    }

    // Read data for a complete FASTQ record from `read`.
    pub fn parse_record<R>(read: &mut R, record: &mut FastqRecord) -> io::Result<()>
    where
        R: BufRead,
    {
        let mut buffer = Vec::new();

        if read.fill_buf()?.is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                "FASTQ file is empty or improperly formatted",
            ));
        }

        // Skip header line (line 1)
        read.read_until(b'\n', &mut buffer)?;
        buffer.clear();

        // Sequence line (line 2)
        read.read_until(b'\n', &mut buffer)?;
        record.seq = buffer.strip_suffix(b"\n").unwrap_or(&buffer).to_vec();
        buffer.clear();

        // Skip '+' line (line 3)
        read.read_until(b'\n', &mut buffer)?;
        buffer.clear();

        // Quality line (line 4)
        read.read_until(b'\n', &mut buffer)?;
        record.qual = buffer.strip_suffix(b"\n").unwrap_or(&buffer).to_vec();

        Ok(())
    }

    pub fn finalize(self) -> Vec<Box<dyn Statistic>> {
        // Move out the statistics, effectively preventing the future use of the runner.
        self.statistics
    }
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