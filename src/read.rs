use std::fs::File;
use std::io::{BufRead, BufReader};

//function for open input file and read the particular file.
pub fn read_file(input:&str) -> BufReader<File> {
    let input_file = File::open(input).expect("Path doesnot already exist !");
    let reader = BufReader::new(input_file);
    return reader;
}

