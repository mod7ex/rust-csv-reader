use std::env::args;
use std::error::Error;
use csv;
use std::process;

fn read_file(path: &str) -> Result<(), Box<dyn Error>> {
    // Build the CSV reader and iterate over each record.
    let mut reader = csv::Reader::from_path(path)?;

    for result in reader.records() {
        // The iterator yields Result<StringRecord, Error>, so we check the
        // error here.
        let record = result?;
        println!("{:?}", record);
    }
    
    Ok(())
}

fn main() {
    let args: Vec<_> = args().collect();

    if args.len() < 2 {
        println!("[USAGE] {} <csv file path>", args[0]);
        return;
    }

    if let Err(e) = read_file(&args[1]) {
        eprintln!("Error running program: {}", e);
        process::exit(1);
    }
}