use std::fs::File;
use std::io::Write;
use std::io::{BufReader, BufRead};

struct Person{
    fname : String,
    lname : String,
    dob : String
}

pub fn task_file(i:BufReader<File>,o:&mut File,mut yyyy:i32,mut mm:i32,dd:i32) {
    for line in i.lines() {
        let l = line.unwrap();
        let words: Vec<&str> = l.split(",").collect();
        let p = Person { 
            fname:String::from(words[0]),
            lname:String::from(words[1]),
            dob: String::from(words[2])
           };

        let bday: Vec<&str> = (p.dob).split("-").collect();

        let y: i32 = bday[0].parse().unwrap();
        let m: i32 = bday[1].parse().unwrap();
        let d: i32 = bday[2].parse().unwrap();

        if d > dd {
            mm = mm - 1 ;
        }
        if m > mm {
            yyyy = yyyy - 1 ;
        }
        let age = (yyyy - y).to_string() ;
        write_file(p.fname, p.lname, age,o);
    }
}

fn write_file(fname:String, lname:String, age:String,o:&mut std::fs::File){
    o.write_all(lname.as_bytes()).expect("`lname` couldn't write into file");
    o.write_all(",".as_bytes()).expect("`,` couldn't write into file");
    o.write_all(fname.as_bytes()).expect("`fname` couldn't write into file");
    o.write_all(",".as_bytes()).expect("`,` couldn't write into file");
    o.write_all(age.as_bytes()).expect("`age` couldn't write into file");
    o.write_all(b"\n");
}