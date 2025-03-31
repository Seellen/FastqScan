use clap::Parser;
use fastq_scan::{
    runner::WorkflowRunner,
    statistics::{
        base_count_per_pos::BaseCountPerPos, base_count_per_read::BaseCountRead,
        phred_per_pos::PhredPerPos, phred_per_read::PhredPerRead, read_data::ReadData,
    },
    utils::process_fastq,
};
use std::{
    fs::File,
    path::{Path, PathBuf},
};

#[derive(Parser, Debug)]
#[command(
    name = "FASTQ Parser",
    version = "1.0",
    about = "Liest FASTQ-Dateien ein"
)]
pub struct Args {
    /// Pfad zur R1 FASTQ-Datei
    #[arg(short = '1', long)]
    pub r1: PathBuf,

    /// Pfad zur R2 FASTQ-Datei (optional für Single-End)
    #[arg(short = '2', long)]
    pub r2: Option<PathBuf>,
}

fn main() {
    // Parse command line arguments
    let args = Args::parse();

    // Calling for Arg 1 and possibly Arg2
    process_file(&args.r1, 1);
    if let Some(read2_path) = args.r2 {
        process_file(&read2_path, 2);
    }

    println!("\n\nFertig. Exiting now!");
}

fn process_file(path: &PathBuf, number: u8) {
    // Check if the file exists
    if !Path::new(path).exists() {
        eprintln!("Fehler: Die Read{}-'{:?}' existiert nicht!", number, path);
        std::process::exit(1);
    } else {
        println!("\nRead{}-Datei: {:?}", number, path);
    }

    // Create the runner
    let mut runn = WorkflowRunner {
        statistics: vec![
            Box::new(BaseCountPerPos::new()),
            Box::new(BaseCountRead::new()),
            Box::new(PhredPerPos::new()),
            Box::new(PhredPerRead::new()),
            Box::new(ReadData::new()),
        ],
    };

    // Process the FASTQ file
    println!("Processing {:?}...", path);
    runn.process(process_fastq(path.to_path_buf()));

    println!("Read has been processed! Printing to file...");

    // get statistics back
    let stats = runn.finalize();

    // Create output file
    let mut file = File::create(format!("output{}.json", number)).expect("Unable to create file");

    // Serialize the statistics to Json
    serde_json::to_writer_pretty(&mut file, &stats).expect("Failed to write to Json");

    println!("Finished printing to file output{}.json", number);
}
