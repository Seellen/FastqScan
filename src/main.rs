use std::io;
mod phred; 



fn main() {
    println!("Please input ur Phred seq:");

    let mut phred_seq = String::new();

    io::stdin()
        .read_line(&mut phred_seq)
        .expect("Failed to read line");

    phred_seq = phred_seq.trim().to_string();

    match phred::avg_qual(&phred_seq) {
        Some(avg) => println!("The average Phred Score of your sequence is: {avg}"),
        None => println!("Invalid Char found in String"),
    }

}
