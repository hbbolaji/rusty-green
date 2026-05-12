extern crate chrono;
use std::{
    ops::Sub,
    thread,
    time::{Duration, Instant},
};

use chrono::NaiveDate;

pub fn test_std_time() {
    let duration_1 = Duration::from_secs(15);
    println!("{:?}", duration_1.as_millis());

    let duration_2 = Duration::from_millis(14500);
    let duration_3 = duration_1.checked_sub(duration_2).unwrap_or_default();
    println!("{}", duration_3.as_millis());

    let now = Instant::now();
    thread::sleep(Duration::from_millis(200));
    println!("{:?}", now.elapsed().as_millis());
}

pub fn test_chrono() {
    let local_now = chrono::Local::now();
    println!("UTC time: {}", local_now.format("%A %d %Y, %B: %H"));

    // let date_1 = NaiveDate::from_isoywd_opt(1997, 43, chrono::Weekday::Thu).unwrap();
    // println!("Day of the year is {}", date_1.format("%A %d %Y, %B: %H"))
}
