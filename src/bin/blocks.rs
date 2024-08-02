use std::io::Read;
use std::time::{Instant, SystemTime};

use mempool_space::blocking;
use reqwest::Url;

// use ureq::get;

const URL: &str = "https://mempool.space/api/blocks";
// const URL: &str = "https://mempool.space/api/blocks/tip";
// const URL: &str = "https://mempool.space/api/blocks/tip/height";

fn main() {
    let n = 1;
    {
        let start = Instant::now();
        let blocks = String::from("blocks");
        let res = blocking(&blocks);
        println!("blocking {:?} {:?} bytes", start.elapsed(), res);
    }
}
