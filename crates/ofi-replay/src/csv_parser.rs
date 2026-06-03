use common::types::TopOfBook;
use csv;
use std::{error::Error, path::Path};

// Will do csv parsing for tob data
// loop for now will read data -> parse(write ? ) -> process
// line by line stream parsing instead of loading full in mem and  then processing
// we can use serde first and then later direct bytes parsing ?

pub fn parse(path: &Path) -> Result<(), Box<dyn Error>> {
    // build the reader
    let mut rdr = csv::Reader::from_path(path)?;
    for result in rdr.deserialize() {
        let record: TopOfBook = result?;
        println!("{:?}", record);
    }

    Ok(())
}
