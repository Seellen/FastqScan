use crate::runner::FastqRecord;
use crate::runner::Output;
use crate::runner::Statistic;
use crate::utils::calculate_phred;
use std::io::Write;
use gnuplot::AxesCommon;
use gnuplot::Figure;

/// Computes mean base quality for a position read.
#[derive(Default)]
pub struct PhredPerPos {
    phred_sums: Vec<f32>,
    amounts: Vec<u64>,
}

impl PhredPerPos {
    pub fn new() -> Self {
        PhredPerPos {
            phred_sums: Vec::new(),
            amounts: Vec::new(),
        }
    }
}

impl Output for PhredPerPos {
    fn out(&self, writer: &mut dyn Write) {
        let qual_avg: Vec<f32> = self
            .phred_sums
            .iter()
            .zip(&self.amounts)
            .map(|(&sum, &amount)| sum / amount as f32)
            .collect();

        let positions: Vec<usize> = (0..qual_avg.len()).collect(); // X-axis: positions
        let qual_values = &qual_avg; // Y-axis: quality scores

        println!("{}", qual_avg[1]);

        let mut fg = Figure::new();
        fg.axes2d()
            .set_title("Base Quality per Position", &[])
            .set_y_range(gnuplot::AutoOption::Fix(0.0), gnuplot::AutoOption::Fix(38.0))
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

impl Statistic for PhredPerPos {
    fn process(&mut self, record: &FastqRecord) {
        // Ensure vectors are large enough
        if self.phred_sums.len() < record.qual.len() {
            self.phred_sums.resize(record.qual.len(), 0.0);
            self.amounts.resize(record.qual.len(), 0);
        }

        // Convert ASCII qualities to Phred scores and sum
        for (i, &qual) in record.qual.iter().enumerate() {
            if let Some(phred) = calculate_phred(qual) {
                self.phred_sums[i] += phred;
                self.amounts[i] += 1;
            } // Convert ASCII to Phred
        }
    }
}
