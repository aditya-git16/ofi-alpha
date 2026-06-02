use std::path::Path;

use ofi_replay::csv_parser::parse;

fn main() {
    let file_path = "../../test/test.csv";
    if let Err(err) = parse(Path::new(file_path)){
        println!("error parsing {}" , err);
    }
}
