extern crate clap;
use clap::{Arg, App};

mod read_file;
mod write_file;
mod age_calculator;
mod current_date;

fn main () {

       let matches = App::new("First Exercise")
                     .version("1.0")
                    .author("athulya")
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
    let input = read_file::read(input_file);
    let mut output = write_file::write(output_file);
    let (yyyy, mm, dd) = current_date::today();
    age_calculator::age_calculator(input,&mut output,yyyy,mm,dd);
}

