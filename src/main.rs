extern crate clap;
use clap::{Arg, App};

mod read;
mod create;
mod calculation;

//Programm execution starts from main.
//clap is used in this function to parse and validate the string of command line arguments.
fn main () {
       let matches = App::new("Second Exercise")
                     .version("1.0")
                    .author("Athulya")
                    .about("argument passing")
            .arg(Arg::with_name("input")
                    .short("i")
                    .long("input")
                    .takes_value(true))
            .arg(Arg::with_name("output")
                     .short("o")
                     .long("output")
                     .takes_value(true))
       .get_matches();

    let input_file = matches.value_of("input").expect("Path does not already exist !");
    let output_file = matches.value_of("output").expect("Output file could not be created");
    let input = read::read_file(input_file);
    let mut output = create::create_file(output_file);
    calculation::calculation(input,&mut output);
}

