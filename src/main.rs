extern crate clap;
use clap::{Arg, App};

mod readfile;
mod writefile;
mod agecalc;
mod today;

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

    let infile = matches.value_of("input").expect("Path does not already exist !");
    let outfile = matches.value_of("output").expect("Output file could not be created");
    let i = readfile::read_file(infile);
    let mut o = writefile::write_file(outfile);
    let (yyyy, mm, dd) = today::today();
    agecalc::task_file(i,&mut o,yyyy,mm,dd);
}

