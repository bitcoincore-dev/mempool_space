use std::io::Read;
use std::time::{Instant, SystemTime};

use mempool_space::blocking;
use reqwest::Url;

// use ureq::get;

const URL: &str = "https://mempool.space/api/address/1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa/txs/mempool";

fn main() {
    let n = 1;
    {
        let start = Instant::now();
        let address_txs_mempool = String::from("/address/");
        let res = blocking(&address_txs_mempool);
        println!("blocking {:?} {:?} bytes", start.elapsed(), res);
    }
}


