use std::fs::File;
use std::io::Write;
extern crate chrono;
use chrono::{DateTime, Utc};
use std::io::{BufReader, BufRead};

struct Person{
    first_name : String,
    last_name : String,
    date_of_birth : String
}

//function for iterate each line of input file , retrieve the current Date and calculated age.
pub fn age_calculator(input:BufReader<File>,output:&mut File) {
    for line in input.lines() {
        let line = line.unwrap();
        let words: Vec<&str> = line.split(",").collect();
        let person = Person { 
            first_name:String::from(words[0]),
            last_name:String::from(words[1]),
            date_of_birth: String::from(words[2])
           };

        let now: DateTime<Utc> = Utc::now();
        let mut yyyy = now.format("%Y").to_string().parse::<i32>().expect("%Y format not found !");
        let mut mm = now.format("%m").to_string().parse::<i32>().expect("%m format not found !");
        let dd = now.format("%d").to_string().parse::<i32>().expect("%d format not found !");
           
        let birthday: Vec<&str> = (person.date_of_birth).split("-").collect();
        let year: i32 = birthday[0].parse().unwrap();
        let month: i32 = birthday[1].parse().unwrap();
        let day: i32 = birthday[2].parse().unwrap();

        if day > dd {
            mm = mm - 1 ;
        }
        if month > mm {
            yyyy = yyyy - 1 ;
        }
        let age = (yyyy - year).to_string() ;
        write(person.first_name, person.last_name, age,output);
    }
}

//function for write the data into text file .
fn write(first_name:String, last_name:String, age:String,output:&mut std::fs::File){
    output.write_all(last_name.as_bytes()).expect("`lname` couldn't write into file");
    output.write_all(",".as_bytes()).expect("`,` couldn't write into file");
    output.write_all(first_name.as_bytes()).expect("`fname` couldn't write into file");
    output.write_all(",".as_bytes()).expect("`,` couldn't write into file");
    output.write_all(age.as_bytes()).expect("`age` couldn't write into file");
    output.write_all(b"\n");
}

