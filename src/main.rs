use clap::Parser;
use fastq_scan::{
    runner::WorkflowRunner,
    statistics::{
        base_qual_pos_stat::BaseQualityPosStatistic, gc_per_read::GcPerRead, nuc_table::NucTable,
        read_qual_stat::ReadQualityStatistic,
    },
    utils::process_fastq,
};
use std::path::{Path, PathBuf};

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
    // let args = Args::parse();
    let args: Args = Args {
        r1: "data/example.R1.fastq.gz".trim_matches('"').into(),
        r2: Some("data/example.R2.fastq.gz".trim_matches('"').into()),
    };
    //let mut r1 = false;
    let mut r2 = false;

    // ---- READ 1 -----
    // Check if the File exists
    if !Path::new(&args.r1).exists() {
        eprintln!("Fehler: Die R1-Datei '{:?}' existiert nicht!", args.r1);
        std::process::exit(1);
    } else {
        println!("\nR1-Datei: {:?}", args.r1);
        //    r1 = true;
    }

    // ---- READ 2 -----
    if let Some(r_2) = &args.r2 {
        // Check if File exists
        if !Path::new(&r_2).exists() {
            eprintln!("Fehler: Die R2-Datei '{:?}' existiert nicht!", r_2);
            std::process::exit(1);
        }
        println!("\nR2-Datei: {:?}", r_2);
        r2 = true;
    } else {
        println!("Nur Single-End Datei angegeben.");
    }

    let mut runner = WorkflowRunner {
        statistics: vec![
            Box::new(BaseQualityPosStatistic::new()),
            Box::new(ReadQualityStatistic::new()),
            Box::new(NucTable::new()),
            Box::new(GcPerRead::new()),
        ],
    };

    if r2 {
        println!("Processing Read 1...");
        runner.process(process_fastq(args.r1));
        println!("Processing Read 2...");
        runner.process(process_fastq(args.r2.expect("File 2 not here")));
    } else {
        println!("Processing Read 1...");
        runner.process(process_fastq(args.r1));
    }

    /*
    main_menu(args,r1,r2);

    // We dont want annoying warnings
    if false {

        getinfo::info_data(&"test".to_string());
        getinfo::info_data(&"test2".to_string());

        calls::phred_call();

        calls::info_call();
    }
    */
}
