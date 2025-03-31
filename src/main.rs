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

//let args: Args = Args {
//    r1: "data/example.R1.fastq.gz".trim_matches('"').into(),
//    r2: Some("data/example.R2.fastq.gz".trim_matches('"').into()),
//};

//let file = File::create("output.json").expect("Unable to create file");
//let mut json_writer: Serializer<_, serde_json::ser::PrettyFormatter<'_>> =  Serializer::pretty(file);

fn main() {
    let args = Args::parse();

    process_file(&args.r1, 1);
    if let Some(read2_path) = args.r2 {
        process_file(&read2_path, 2);
    }

    println!("\n\nFertig. Exiting now!");
}

fn process_file(path: &PathBuf, number: u8) {
    if !Path::new(path).exists() {
        eprintln!("Fehler: Die Read{}-'{:?}' existiert nicht!", number, path);
        std::process::exit(1);
    } else {
        println!("\nRead{}-Datei: {:?}", number, path);
    }

    println!("Processing {:?}...", path);

    let mut runn = WorkflowRunner {
        statistics: vec![
            Box::new(BaseCountPerPos::new()),
            Box::new(BaseCountRead::new()),
            Box::new(PhredPerPos::new()),
            Box::new(PhredPerRead::new()),
            Box::new(ReadData::new()),
        ],
    };

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
