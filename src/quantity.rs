use flate2::read::GzDecoder;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn count_nucleotides(reader: BufReader<GzDecoder<File>>) -> Vec<HashMap<char, f32>> {
    let mut sum = 0; // Initialize sum

    // Convert the non-cloneable lines iterator into one that we can manipulate
    let mut lines = reader.lines();

    // Discard the header line (first line)
    lines.next();

    // Get the first sequence line (which will be used to compute seq_len)
    let first_seq_line = match lines.next() {
        Some(Ok(line)) => line,
        Some(Err(e)) => panic!("Error reading the sequence line: {}", e),
        None => panic!("No sequence line found"),
    };

    // Compute sequence length using the trimmed sequence line
    let seq_len = first_seq_line.trim_matches('"').len();
    // Reconstruct an iterator that starts with the first sequence line, then the rest
    let all_lines = std::iter::once(Ok(first_seq_line)).chain(lines);

    let mut counts: Vec<HashMap<char, f32>> = vec![HashMap::new(); seq_len];

    for (index, line) in all_lines.enumerate() {
        if index % 4 == 0 {
            if let Ok(line) = line {
                let trimmed = line.trim_matches('"');
                // Skip if the line doesn't match our expected length
                if trimmed.len() != seq_len {
                    continue;
                }
                for (i, base) in trimmed.chars().enumerate() {
                    let counter = counts[i].entry(base).or_insert(0.0);
                    *counter += 1.0;
                }
                sum += 1;
            }
        }
    }

    for count in &mut counts {
        for (_, value) in count.iter_mut() {
            *value /= sum as f32; // Normalize counts
        }
    }

    counts
}

pub fn nucleotide_table(counts: Vec<HashMap<char, f32>>) {
    println!("Position\tA(%)\tC(%)\tG(%)\tT(%)");
    for (i, count_map) in counts.iter().enumerate() {
        let a_freq = *count_map.get(&'A').unwrap_or(&0.0) * 100.0;
        let c_freq = *count_map.get(&'C').unwrap_or(&0.0) * 100.0;
        let g_freq = *count_map.get(&'G').unwrap_or(&0.0) * 100.0;
        let t_freq = *count_map.get(&'T').unwrap_or(&0.0) * 100.0;

        println!(
            "{}\t\t{:.2}%\t{:.2}%\t{:.2}%\t{:.2}%",
            i + 1,
            a_freq,
            c_freq,
            g_freq,
            t_freq
        );
    }
    println!();
}
