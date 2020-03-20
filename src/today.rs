extern crate chrono;
use chrono::{DateTime, Utc};

pub fn today() -> (i32,i32,i32) {
    let now: DateTime<Utc> = Utc::now();
    let yyyy = now.format("%Y").to_string().parse::<i32>().expect("%Y format not found !");
    let mm = now.format("%m").to_string().parse::<i32>().expect("%m format not found !");
    let dd = now.format("%d").to_string().parse::<i32>().expect("%d format not found !");
    return (yyyy,mm,dd);
}
