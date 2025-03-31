use crate::runner::{FastqRecord, Statistic};
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]
pub struct ReadData {
    read_lengths: Vec<(usize, usize)>,
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

        let len = record.seq.len();
        if let Some(pos) = self.read_lengths.iter().position(|(l, _)| *l == len) {
            self.read_lengths[pos].1 += 1;
        } else {
            self.read_lengths.push((len, 1));
        }
    }
}
