use regex::Regex;

const DATA_PATTERN: &str = r"[_\.]";
const READ_PATTERN: &str = r"[ :\.]";

#[derive(Debug)]
struct DataInfo{
    sample_name: String,
    barcode_sequence: String,
    lane_number: usize,
    read_number: usize,
    set_number: usize,
}

impl DataInfo {
    
    // Constructor:
    pub fn new(info: Vec<&str>) -> Result<Self,String> {
        
        if info.len() < 5{
            return Err("Data is missing Information".to_string());
        }

        // check for conversion errors
        let lane_number = info[2][1..].parse::<usize>().map_err(|_| "Failed to parse lane_number")?;
        let read_number = info[3][1..].parse::<usize>().map_err(|_| "Failed to parse read_number")?;
        let set_number = info[4].parse::<usize>().map_err(|_| "Failed to parse set_number")?;

        // put info into new struct
        Ok(DataInfo{
            sample_name : info[0].to_string(),
            barcode_sequence: info[1].to_string(),
            lane_number,
            read_number,
            set_number,        })
    }

    pub fn display(&self){
        println!("\nHier die übersicht über deine data file!");
        println!("Name of the sample: {}", self.sample_name);
        println!("Barcode sequence used for multiplexing: {}", self.barcode_sequence);
        println!("Lane number (1 -8): {}", self.lane_number);
        println!("Read number (either 1 or 2): {}", self.read_number);
        println!("Set number: {}", self.set_number);
    }
}

#[derive(Debug)]
struct ReadInfo{
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
    pub fn new(info: Vec<&str>) -> Result<Self,String> {
        
        if info.len() < 11 {
            return Err("Data is missing Information".to_string());
        }

        // Check for conversion errors
        let run = info[1].parse::<usize>().map_err(|_| "Failed to parse lane_number")?;
        let lane = info[3].parse::<usize>().map_err(|_| "Failed to parse read_number")?;
        let tile_number = info[4].parse::<usize>().map_err(|_| "Failed to parse set_number")?;
        let x_pos = info[5].parse::<usize>().map_err(|_| "Failed to parse read_number")?;
        let y_pos = info[6].parse::<usize>().map_err(|_| "Failed to parse read_number")?;
        let read = info[7].parse::<usize>().map_err(|_| "Failed to parse read_number")?;
        let is_filtered = info[8].parse::<char>().map_err(|_| "Failed to parse read_number")?;
        let control_number = info[9].parse::<usize>().map_err(|_| "Failed to parse read_number")?;

        // put info into new struct
        Ok(ReadInfo{
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

    pub fn display(&self){
        println!("\nHier die übersicht über deine read Datei!");
        println!("Die eindeutige Gerätebezeichnung: {}", &self.instrument[1..]);  // Remove leading character
        println!("Die Lauf-ID : {}", self.run);
        println!("Flowcell-ID: {}", self.flowcell_id);
        println!("Flowcell-Lane (Spur: 1–8): {}", self.lane);
        println!("Tile-Nummer innerhalb der Lane: {}", self.tile_number);
        println!("X-Koordinate des Clusters: {}", self.x_pos);
        println!("Y-Koordinate des Clusters: {}", self.y_pos);
        println!("Mitglied eines Paares (1 oder 2): {}", self.read);
        println!("Chastity Filter information: {}", self.is_filtered);
        println!("Kontrollbits: {}", self.control_number);
        println!("Indexsequenz: {}", self.index);
    }
    
}

pub fn info_data(data: &String){

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

pub fn info_read(data: &String){

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

// Split string based on parameters
fn split_data<'a>(data: &'a str, pattern: &str) -> Result<Vec<&'a str>, regex::Error> {
    let re = Regex::new(pattern)?;  // Compile the regex pattern
    Ok(re.split(data).collect())    // Split the data and return as a Vec
}
