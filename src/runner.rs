use std::io::{self, BufRead};
use std::fmt::Write;


#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct FastqRecord {
    pub seq: Vec<u8>,
    pub qual: Vec<u8>,
}

pub trait Statistic: Output {
    /* Statistics:

    * average base quality (Phred)
    * average quality of all reads
    * average proportions of `{A, C, G, T, N}` for each read position
    * ...
    */

    fn process(&mut self, record: &FastqRecord);
    // TODO - find a way to represent the results.
    // Let's try to identify the shared parts of *any* statistic
    // and report these in some fashion.
    // fn report(self) -> ?
}

pub trait Output {
    fn out(&self, writer: &mut dyn Write);
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

        for statistic in self.statistics.iter() {
            statistic.out();
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
