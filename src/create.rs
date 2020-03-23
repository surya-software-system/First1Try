use std::fs::File;

//function for creating a new file and return the file
pub fn create_file(output:&str) -> File{
    let output_file = File::create(output).expect("Output file could not be created !");
    return output_file;
}

