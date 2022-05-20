use std::process;
use std::error::Error;

use chrono::Local;
use csv::StringRecord;

const FILEPATH: &str = "./logfile.csv";

fn main() {
    let datetime = Local::now().to_string();

    if let Err(err) = csv_writer(&datetime) {
        println!("error writing csv: {}", err);
        process::exit(1);
    }
    if let Err(err) = csv_reader() {
        println!("error reading csv: {}", err);
        process::exit(1);
    }
}

fn csv_writer(data: &String) -> Result<(), Box<dyn Error>> {
    // write to stdout
    // let mut wtr = csv::Writer::from_writer(io::stdout());

    // write to file
    let mut wtr = csv::Writer::from_path(FILEPATH)?;

    // TODO how to append to existing file?
    wtr.write_record(&["-->".to_string(), data.to_string()])?;
    wtr.flush()?;

    Ok(())
}

fn csv_reader() -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false) // read with headers
        .from_path(FILEPATH)?;
    let rec = rdr.records().collect::<Result<Vec<StringRecord>, csv::Error>>()?;
    println!("{:?}", rec);

    // for result in rdr.records() {
    //     let record = result?;
    //     println!("{:?}", record);
    // }

    Ok(())
}
