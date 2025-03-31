use crate::runner::{FastqRecord, Statistic};
use crate::utils::{ask_for_len, calculate_phred};
use gnuplot::AxesCommon;
use gnuplot::Figure;
use serde::{Deserialize, Serialize};

/// Computes mean base quality for a read.
#[derive(Default, Serialize, Deserialize)]
pub struct PhredPerRead {
    mean: Vec<f32>,
}

impl PhredPerRead {
    pub fn new() -> Self {
        PhredPerRead { mean: Vec::new() }
    }

    fn _out(&self) {
        let read_nr: Vec<usize> = (0..self.mean.len()).collect(); // X-axis: positions
        let qual_values = &self.mean; // Y-axis: quality scores

        println!("\nPlotting the average quality per read");

        let start = match ask_for_len("\n   Input start position if desired: ") {
            Ok(num) => num,
            Err(_) => {
                println!("\tInvalid start position starting at 0\n");
                0
            }
        };

        let end = match ask_for_len("   Input end position if desired: ") {
            Ok(num) => num,
            Err(_) => {
                println!("\tInvalid end position ending at the last read");
                qual_values.len() as u32 - 1
            }
        };

        let (plot_start, plot_end) = if start <= end && end < qual_values.len() as u32 {
            (start as usize, end as usize)
        } else {
            println!("Invalid range, displaying full plot");
            (0, qual_values.len() - 1)
        };

        if start <= end && end < qual_values.len() as u32 {
            let mut fg = Figure::new();
            fg.axes2d()
                .set_title("Avg Quality per Read", &[])
                .set_x_label("Read Number", &[])
                .set_y_label("Average Quality Score", &[])
                .lines(
                    read_nr[plot_start..plot_end].to_vec(),
                    qual_values[plot_start..plot_end].to_vec(),
                    &[gnuplot::Caption("Avg Quality"), gnuplot::Color("blue")],
                );

            fg.show().unwrap(); // Display the plot
        } else {
            let mut fg = Figure::new();
            fg.axes2d()
                .set_title("Avg Quality per Read", &[])
                .set_x_label("Read Number", &[])
                .set_y_label("Average Quality Score", &[])
                .lines(
                    read_nr,
                    qual_values,
                    &[gnuplot::Caption("Avg Quality"), gnuplot::Color("blue")],
                );

            fg.show().unwrap(); // Display the plot
        }
    }
}

#[typetag::serde]
impl Statistic for PhredPerRead {
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
        }
    }
}
