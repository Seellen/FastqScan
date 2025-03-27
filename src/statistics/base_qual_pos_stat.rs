use crate::runner::Statistic;
use crate::runner::FastqRecord;
use crate::utils::calculate_phred;
use gnuplot::AxesCommon;
use gnuplot::Figure;

/// Computes mean base quality for a position read.
pub struct BaseQualityPosStatistic {
    qual_sums: Vec<f32>,
    amounts: Vec<u64>,
}

impl BaseQualityPosStatistic {
    pub fn new() -> Self {
        BaseQualityPosStatistic {
            qual_sums: Vec::new(),
            amounts: Vec::new(),
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

    fn display(&self) {

        let qual_avg: Vec<f32> = self
        .qual_sums
        .iter()
        .zip(&self.amounts)
        .map(|(&sum, &amount)| sum / amount as f32)
        .collect();

        let positions: Vec<usize> = (0..qual_avg.len()).collect(); // X-axis: positions
        let qual_values = &qual_avg; // Y-axis: quality scores

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