use std::fs::File;
use std::io::BufReader;

pub fn read_file(i:&str) -> BufReader<File>{
    let input_file = File::open(i).expect("Path does not already exist !");
    let r = BufReader::new(input_file);
    return r;
}