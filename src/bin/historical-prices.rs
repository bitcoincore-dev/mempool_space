use std::io::Read;
use std::time::{Instant, SystemTime};

use mempool_space::blocking;
use reqwest::Url;

// use ureq::get;

const URL: &str = "https://mempool.space/api/v1/historical-price";

fn main() {
    let n = 1;
    {
        let start = Instant::now();
        let historical_price = &String::from("historical-price");
        let res_format = format!("/v1/{}", historical_price);
        let res = blocking(&res_format);
        println!("blocking {:?} {:?} bytes ", start.elapsed(), res);
    }
}
