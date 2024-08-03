use std::io::Read;
use std::time::{Instant, SystemTime};

use mempool_space::blocking;
use reqwest::Url;

// use ureq::get;

const URL: &str = "https://mempool.space/api/address/1KFHE7w8BhaENAswwryaoccDb6qcT6DbYY/utxo";

fn main() {
    let n = 1;
    {
        let start = Instant::now();
        let address_utxo = String::from("/address");
        let res = blocking(&address_utxo);
        println!("blocking {:?} {:?} bytes", start.elapsed(), res);
    }
}
