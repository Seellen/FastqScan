use clap::Parser;
use std::path::{Path, PathBuf};
use std::io::{BufRead, BufReader};
use std::fs::File;
use crate::phred::avg_qual;
use flate2::read::GzDecoder;
use prettytable::{Table, row, cell};
mod phred; 
mod getinfo;    
mod calls;
#[cfg(test)]
mod tests;

#[derive(Parser)]
#[command(name = "FASTQ Parser", version = "1.0", about = "Liest FASTQ-Dateien ein")]

struct Args {
    /// Pfad zur R1 FASTQ-Datei
    #[arg(short = '1', long)]
    r1: PathBuf,

    /// Pfad zur R2 FASTQ-Datei (optional für Single-End)
    #[arg(short = '2', long)]
    r2: Option<PathBuf>,
}

fn process_fastq(file_path: PathBuf) {
    // open and use file
    let file = File::open(file_path).expect("Konnte die Datei nicht öffnen");
    let decoder = GzDecoder::new(file); // Entpacke Gzip
    let reader = BufReader::new(decoder);

    let mut table = Table::new();
    table.add_row(row!["Position", "Avg Phred score"]);

    let mut line_count = 0;
    for line in reader.lines().filter_map(Result::ok) {
        line_count += 1;
        if line_count % 4 == 0 {  // Qualitätsscore-Zeilen in FASTQ-Dateien sind jede 4. Zeile
            let avg_score = avg_qual(line.as_bytes());
            table.add_row(row![line_count/4,format!("{:?}", avg_score)]);
        }
    }

    table.printstd();
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
    
    println!("\nR1-Datei: {:?}", args.r1);
    process_fastq(args.r1);
    
    // ---- READ 2 -----
    if let Some(r2) = args.r2 {
        // Check if File exists
        if !Path::new(&r2).exists() {
            eprintln!("Fehler: Die R2-Datei '{:?}' existiert nicht!", r2);
            std::process::exit(1);
        }
        _read2 = r2.clone();
        println!("\nR2-Datei: {:?}", r2);
        process_fastq(_read2);
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
