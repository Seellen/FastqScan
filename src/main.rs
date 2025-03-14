use std::io;
mod phred; 
mod getinfo;



fn main() {
    println!("\nPlease input ur Phred seq:");

    let mut phred_seq = String::new();

    io::stdin()
        .read_line(&mut phred_seq)
        .expect("Failed to read line");

    phred_seq = phred_seq.trim().to_string();

    match phred::avg_qual(&phred_seq) {
        Some(avg) => println!("The average Phred Score of your sequence is: {avg}"),
        None => println!("Invalid Char found in String"),
    }

    let mut x = "NIST7035_TAAGGCGA_L001_R1_001.fastq.gz".to_string();
    let mut y = "@HWI-D00119:50:H7AP8ADXX:1:1101:2100:2202 1:N:0:TAAGGCGA".to_string();
    getinfo::info_data(&mut x);
    getinfo::info_read(&mut y);

}
