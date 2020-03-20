use std::fs::File;
use std::io::BufReader;

//function for open input file and read the perticular file.
pub fn read(input:&str) -> BufReader<File>{
    let input_file = File::open(input).expect("Path does not already exist !");
    let reader = BufReader::new(input_file);
    return reader;
}

