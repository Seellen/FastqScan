use crate::runner::{FastqRecord, Statistic};
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]
pub struct ReadData {
    read_lengths: Vec<usize>,
}

impl ReadData {
    pub fn new() -> Self {
        ReadData {
            read_lengths: Vec::new(),
        }
    }
}

#[typetag::serde]
impl Statistic for ReadData {
    fn process(&mut self, record: &FastqRecord) {
        self.read_lengths.push(record.seq.len());
    }
}
