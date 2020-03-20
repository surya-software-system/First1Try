extern crate chrono;
use chrono::{DateTime, Utc};
struct Today {
    yyyy : i32,
    mm : i32,
    dd : i32
}

//function for get current date in the form of yyyy-mm-dd 
pub fn today() -> (i32,i32,i32) {
    let now: DateTime<Utc> = Utc::now();
    let today = Today {
        yyyy : now.format("%Y").to_string().parse::<i32>().expect("%Y format not found !"),
        mm : now.format("%m").to_string().parse::<i32>().expect("%m format not found !"),
        dd : now.format("%d").to_string().parse::<i32>().expect("%d format not found !")
    
    };
    return (today.yyyy, today.mm, today.dd);
}

