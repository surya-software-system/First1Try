use std::fs::File;
use std::io::Write;
use std::str::FromStr;
use std::collections::HashMap;
use std::io::{BufReader,BufRead};

struct Person{
    first_name : String,
    amount : i32
}

//function for iterating each line in input file , separating each word based 
//on comma and store into a vector. And for creating a hashmap .
pub fn calculation(input:BufReader<File>,output:&mut File) {
    let mut list = HashMap::new();
    for line in input.lines() {
        let line = line.unwrap();
        let words: Vec<&str> = line.split(",").collect();
        
        let mut first_name = words[0].to_string();
        let amount :i32 = FromStr::from_str(words[2]).expect("Index out of bound !");
        fill_list(&mut first_name,amount,&mut list);
    }
    write_file(&mut list,output);
}

//function for filling hashmap based on some conditions.
fn fill_list(first_name:&mut String, amount:i32, list: &mut HashMap<String, i32>){
    if list.contains_key(first_name){
        let add_amount = amount + list.get(first_name).expect("There is no reference to the value corresponding to the key !");
        list.insert(first_name.to_string(),add_amount);
    }
    else {
         list.insert(first_name.to_string(),amount);
    }   
}

//function for writting datas to the text file.
fn write_file(list: &mut HashMap<String, i32>, file : &mut std::fs::File){
    for (key,val) in list.iter(){
        let person = Person {
            first_name : key.to_string(),
            amount : *val,
        };
        file.write_all(person.first_name.as_bytes()).expect("`person.first_name` couldn't write into file");
        file.write_all(",".as_bytes()).expect("`,` couldn't write into file");
        file.write_all((person.amount).to_string().as_bytes()).expect("`person.amount` couldn't write into file");
        file.write(b"\n");
    }     
}

