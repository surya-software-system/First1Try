use std::fs::File;

pub fn write_file (o:&str) -> File {
    let output_file = File::create(o).expect("Output file could not be created");
    return output_file;
}
