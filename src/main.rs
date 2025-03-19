use clap::{ArgGroup, Parser};
use std::path::{Path, PathBuf};
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
    r1: PathBuf,

    /// Pfad zur R2 FASTQ-Datei (optional für Single-End)
    #[arg(short = '2', long)]
    r2: Option<PathBuf>,
}




fn main() {

    let _read2: PathBuf;
    let args = Args::parse();

    // ---- READ 1 -----
    // Check if the File exist
    if !Path::new(&args.r1).exists() {
        eprintln!("Fehler: Die R1-Datei '{:?}' existiert nicht!", args.r1);
        std::process::exit(1);
    }
    
    println!("R1-Datei: {:?}", args.r1);
    
    // ---- READ 2 -----
    if let Some(r2) = args.r2 {
        // Check if File exists
        if !Path::new(&r2).exists() {
            eprintln!("Fehler: Die R2-Datei '{:?}' existiert nicht!", r2);
            std::process::exit(1);
        }
        println!("R2-Datei: {:?}", r2);
    } else {
        println!("Nur Single-End Datei angegeben.");
    }


    // We dont want annoying warnings
    if false {
        
        getinfo::info_data(&"test".to_string());
        getinfo::info_data(&"test2".to_string());

        calls::phred_call();

        calls::info_call();
    }


}
