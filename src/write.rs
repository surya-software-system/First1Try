use std::fs::File;

//function for creating a new file and return the file
pub fn write (o:&str) -> File {
    let output_file = File::create(o).expect("Output file could not be created");
    return output_file;
}

