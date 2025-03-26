use crate::runner::{Statistic, FastqRecord};
use crate::utils::calculate_phred;
use gnuplot::AxesCommon;
use gnuplot::Figure;

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

