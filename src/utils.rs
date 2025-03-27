use flate2::read::GzDecoder;
use std::fs::File;
use std::io;
use std::io::BufReader;
use std::io::Write;
use std::path::{Path, PathBuf};

// Check if a file exists
pub fn file_exists(file_path: &Path) -> io::Result<()> {
    if !file_path.exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("File '{:?}' not found!", file_path),
        ));
    }
    Ok(())
}

// Split a string by a regex pattern (reusable from `getinfo.rs`)
pub fn split_data<'a>(data: &'a str, pattern: &str) -> Result<Vec<&'a str>, regex::Error> {
    let re = regex::Regex::new(pattern)?;
    Ok(re.split(data).collect())
}

pub fn process_fastq(file_path: PathBuf) -> BufReader<GzDecoder<File>> {
    // open and use file
    if !file_path.exists() {
        panic!("File not found: {:?}", file_path);
    }
    let file_zip = File::open(file_path).expect("Konnte die Datei nicht öffnen");
    let file = GzDecoder::new(file_zip); // Entpacke Gzip
    BufReader::new(file)
}

pub fn ask_for_len(message: &str) -> Result<u32, String> {
    if !message.is_empty() {
        println!("{message}");
    }
    io::stdout().flush().unwrap(); // Ensure prompt is shown

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).unwrap();
    let choice = choice.trim();

    match choice.parse::<u32>() {
        Ok(num) => Ok(num),
        Err(_) => Err("Invalid input: please enter a positive integer".to_string()),
    }
}

// ------------------- PHRED SCORES -------------------
pub fn avg_qual(qual_str: &[u8]) -> Option<f32> {
    if qual_str.is_empty() {
        return None;
    }

    let qu_sum: f32 = qual_str
        .iter()
        .map(|&qu| calculate_phred(qu))
        .collect::<Option<Vec<f32>>>()?
        .iter()
        .sum();

    Some(qu_sum / qual_str.len() as f32)
}

pub fn calculate_phred(qual: u8) -> Option<f32> {
    if (33..=126).contains(&qual) {
        Some((qual as f32) - 33.0)
    } else {
        None
    }
}

// --------------------------- READ/FILE INFO -------------------------
const DATA_PATTERN: &str = r"[_\.]";
const READ_PATTERN: &str = r"[ :\.]";

#[derive(Debug)]
struct DataInfo {
    sample_name: String,
    barcode_sequence: String,
    lane_number: usize,
    read_number: usize,
    set_number: usize,
}

impl DataInfo {
    // Constructor:
    pub fn new(info: Vec<&str>) -> Result<Self, String> {
        if info.len() < 5 {
            return Err("Data is missing Information".to_string());
        }

        // check for conversion errors
        let lane_number = info[2][1..]
            .parse::<usize>()
            .map_err(|_| "Failed to parse lane_number")?;
        let read_number = info[3][1..]
            .parse::<usize>()
            .map_err(|_| "Failed to parse read_number")?;
        let set_number = info[4]
            .parse::<usize>()
            .map_err(|_| "Failed to parse set_number")?;

        // put info into new struct
        Ok(DataInfo {
            sample_name: info[0].to_string(),
            barcode_sequence: info[1].to_string(),
            lane_number,
            read_number,
            set_number,
        })
    }

    pub fn display(&self) {
        println!("\nAn overview of your file!");
        println!("Name of the sample: {}", self.sample_name);
        println!("Barcode sequence: {}", self.barcode_sequence);
        println!("Lane number (1 -8): {}", self.lane_number);
        println!("Read number (1 or 2): {}", self.read_number);
        println!("Set number: {}", self.set_number);
    }
}

#[derive(Debug)]
struct ReadInfo {
    instrument: String,
    run: usize,
    flowcell_id: String,
    lane: usize,
    tile_number: usize,
    x_pos: usize,
    y_pos: usize,
    read: usize,
    is_filtered: char,
    control_number: usize,
    index: String,
}

impl ReadInfo {
    // Constructor:
    pub fn new(info: Vec<&str>) -> Result<Self, String> {
        if info.len() < 11 {
            return Err("Data is missing Information".to_string());
        }

        // Check for conversion errors
        let run = info[1]
            .parse::<usize>()
            .map_err(|_| "Failed to parse lane_number")?;
        let lane = info[3]
            .parse::<usize>()
            .map_err(|_| "Failed to parse read_number")?;
        let tile_number = info[4]
            .parse::<usize>()
            .map_err(|_| "Failed to parse set_number")?;
        let x_pos = info[5]
            .parse::<usize>()
            .map_err(|_| "Failed to parse read_number")?;
        let y_pos = info[6]
            .parse::<usize>()
            .map_err(|_| "Failed to parse read_number")?;
        let read = info[7]
            .parse::<usize>()
            .map_err(|_| "Failed to parse read_number")?;
        let is_filtered = info[8]
            .parse::<char>()
            .map_err(|_| "Failed to parse read_number")?;
        let control_number = info[9]
            .parse::<usize>()
            .map_err(|_| "Failed to parse read_number")?;

        // put info into new struct
        Ok(ReadInfo {
            instrument: info[0].to_string(),
            run,
            flowcell_id: info[2].to_string(),
            lane,
            tile_number,
            x_pos,
            y_pos,
            read,
            is_filtered,
            control_number,
            index: info[10].to_string(),
        })
    }

    pub fn display(&self) {
        println!("\nHier die übersicht über deine read Datei!");
        println!(
            "Die eindeutige Gerätebezeichnung: {}",
            &self.instrument[1..]
        ); // Remove leading character
        println!("Die Lauf-ID : {}", self.run);
        println!("Flowcell-ID: {}", self.flowcell_id);
        println!("Flowcell-Lane (Spur: 1–8): {}", self.lane);
        println!("Tile-Nummer: {}", self.tile_number);
        println!("X-Koordinate: {}", self.x_pos);
        println!("Y-Koordinate: {}", self.y_pos);
        println!("Mitglied eines Paares (1 oder 2): {}", self.read);
        println!("Chastity Filter information: {}", self.is_filtered);
        println!("Kontrollbits: {}", self.control_number);
        println!("Indexsequenz: {}", self.index);
    }
}

pub fn info_data(data: &str) {
    // First we split the File name into its parts
    match split_data(data, DATA_PATTERN) {
        Ok(info) => {
            // We create a new struct and display it
            match DataInfo::new(info) {
                Ok(datainfo) => datainfo.display(),
                Err(e) => println!("Could not display your data: {e}"),
            }
        }
        Err(e) => println!("Couldnt split input name of your data: {e}"),
    }
}

pub fn info_read(data: &str) {
    // First we split the File name into its parts
    match split_data(data, READ_PATTERN) {
        Ok(info) => {
            match ReadInfo::new(info) {
                //We create a new struct and display it
                Ok(read_info) => read_info.display(),
                Err(e) => println!("Couldnt display your read data: {e}"),
            }
        }
        Err(e) => println!("Couldnt split input name of your read: {e}"),
    }
}
