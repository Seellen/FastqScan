use clap::{ArgGroup, Parser};
use std::path::Path;
mod phred; 
mod getinfo;    
mod calls;
#[cfg(test)]
mod tests;

#[derive(Parser)]
#[command(name = "FASTQ Parser", version = "1.0", about = "Liest FASTQ-Dateien ein")]
#[command(group(
    ArgGroup::new("input")
        .required(true)
        .args(["r1", "r2"]),
))]
struct Args {
    /// Pfad zur R1 FASTQ-Datei
    #[arg(short = '1', long)]
    r1: String,

    /// Pfad zur R2 FASTQ-Datei (optional für Single-End)
    #[arg(short = '2', long)]
    r2: Option<String>,
}




fn main() {

    let mut read2: String = "".to_string();
    let args = Args::parse();

    // Check if the File exist
    if !Path::new(&args.r1).exists() {
        eprintln!("Fehler: Die R1-Datei '{}' existiert nicht!", args.r1);
        std::process::exit(1);
    }
    
    println!("R1-Datei: {}", args.r1);
    
    if let Some(r2) = args.r2 {
        // Check if File exists
        if !Path::new(&r2).exists() {
            eprintln!("Fehler: Die R2-Datei '{}' existiert nicht!", r2);
            std::process::exit(1);
        }
        read2 = r2.clone();
        println!("R2-Datei: {}", r2);
    } else {
        println!("Nur Single-End Datei angegeben.");
    }

    getinfo::info_data(&args.r1);
    getinfo::info_data(&read2.to_string());

    calls::phred_call();

    calls::info_call();

}
