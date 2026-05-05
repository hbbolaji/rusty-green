#![allow(dead_code, unused_variables)]

use std::{
    error::Error,
    time::{Duration, Instant},
};

use reqwest::StatusCode;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let start_time = Instant::now();
    tokio::spawn(heart_beat(0));
    let (status_1, status_2) = tokio::join!(
        get_status("https://google.github.io/comprehensive-rust/index.html"),
        get_status("https://google.github.io/comprehensive-rust/types-and-values.html")
    );
    println!("Status 1: {}", status_1.unwrap());
    println!("Status 2: {}", status_2.unwrap());
    println!(
        "Ovarll execuation time: {}ms",
        start_time.elapsed().as_millis()
    );
    Ok(())
}

async fn heart_beat(mut num: u32) {
    loop {
        println!("beating... {}", num);
        tokio::time::sleep(Duration::from_millis(25)).await;
        num += 1;
    }
}

async fn get_status(url: &str) -> Result<StatusCode, Box<dyn Error>> {
    let start_time = Instant::now();
    let status_code = reqwest::get(url).await?.status();
    let duration = start_time.elapsed().as_millis();
    println!("Took {}ms to fetch url '{}'", duration, url);
    Ok(status_code)
}
