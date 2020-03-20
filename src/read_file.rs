use std::fs::File;
use std::io::BufReader;

pub fn read(i:&str) -> BufReader<File>{
    let input_file = File::open(i).expect("Path does not already exist !");
    let r = BufReader::new(input_file);
    return r;
}
//function for open input file and read the particular file.
