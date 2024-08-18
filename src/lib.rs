//! # mempool-space (command line utility)
//!
//! cargo install \--git <https://github.com/RandyMcMillan/mempool_space.git>
//!
//! cargo add \--git <https://github.com/RandyMcMillan/mempool_space.git>
//
//! # CLI: [mempool-space](https://docs.rs/mempool_space/latest/mempool_space) \--option arg<sub>1</sub> ... \--option arg<sub>n</sub>
//!
//!	mempool-space  \--option
//!
//!	mempool-space  \--option arg
//!
//!	mempool-space  \--option arg --option arg
//!
//! # BIN: mempool-space_option arg<sub>1</sub> ... arg<sub>n</sub>
//!
//!	mempool-space_option
//!
//!	mempool-space_option arg
//!
//!	mempool-space_option arg arg
//!
//
//! ## [GENERAL](https://mempool.space/docs/api/rest#get-difficulty-adjustment)
//
//! #### [GET /api/v1/difficulty-adjustment](https://mempool.space/api/v1/difficulty-adjustment)
//!
//!	mempool-space \--difficulty_adjustment
//!
//!	mempool-space_difficulty_adjustment
//!
//! #### [GET /api/v1/prices](https://mempool.space/api/v1/prices)
//!
//!	mempool-space \--prices
//!
//!	mempool-space_prices
//!
//! #### [GET /api/v1/historical-price?currency=EUR&timestamp=1500000000](https://mempool.space/api/v1/historical-price?currency=EUR&timestamp=1500000000)
//!
//!	mempool-space \--historical_price \--currency [USD, CAD, GBP, CHF, AUD, JPY] \--timestamp utc_sec
//!
//!	mempool-space \--historical_price \--currency EUR \--timestamp 1500000000
//!
//!	mempool-space \--historical_price \--currency USD \--timestamp $(date +%s)
//!
//
//! ## [ADDRESSES](https://mempool.space/docs/api/rest#get-address)
//
//!
//! #### [GET /api/address:address](https://mempool.space/api/address/1wiz18xYmhRX6xStj2b9t1rwWX4GKUgpv)
//!
//!	mempool-space \--address 1wiz18xYmhRX6xStj2b9t1rwWX4GKUgpv
//!
//!	mempool-space_address 1wiz18xYmhRX6xStj2b9t1rwWX4GKUgpv
//!
//! #### [GET /api/address:address/txs](https://mempool.space/api/address/1wiz18xYmhRX6xStj2b9t1rwWX4GKUgpv/txs)
//!
//!	mempool-space \--address_txs 1wiz18xYmhRX6xStj2b9t1rwWX4GKUgpv
//!
//!	mempool-space_address_txs 1wiz18xYmhRX6xStj2b9t1rwWX4GKUgpv
//!
//! #### [GET /api/address:address/txs/chain](https://mempool.space/api/address/1wiz18xYmhRX6xStj2b9t1rwWX4GKUgpv/txs/chain)
//!
//!	mempool-space \--address_txs_chain 1wiz18xYmhRX6xStj2b9t1rwWX4GKUgpv
//!
//!	mempool-space_address_txs_chain 1wiz18xYmhRX6xStj2b9t1rwWX4GKUgpv
//!
//! #### [GET /api/address:address/txs/mempool](https://mempool.space/api/address/1wiz18xYmhRX6xStj2b9t1rwWX4GKUgpv/txs/mempool) (may be empty for test address)
//!
//!	mempool-space \--address_txs_mempool 1wiz18xYmhRX6xStj2b9t1rwWX4GKUgpv
//!
//!	mempool-space_address_txs_mempool 1wiz18xYmhRX6xStj2b9t1rwWX4GKUgpv
//!
//! #### [GET /api/address:address/utxo](https://mempool.space/api/address/1wiz18xYmhRX6xStj2b9t1rwWX4GKUgpv/utxo)
//!
//!	mempool-space \--address_utxo 1wiz18xYmhRX6xStj2b9t1rwWX4GKUgpv
//!
//!	mempool-space_address_utxo 1wiz18xYmhRX6xStj2b9t1rwWX4GKUgpv
//!
//! #### [GET /api/v1/validate-address/:address](https://mempool.space/api/v1/validate-address/1KFHE7w8BhaENAswwryaoccDb6qcT6DbYY)
//!
//!	mempool-space \--validate_address 1KFHE7w8BhaENAswwryaoccDb6qcT6DbYY
//!
//!	mempool-space_validate_address 1KFHE7w8BhaENAswwryaoccDb6qcT6DbYY
//
//! ## [BLOCKS](https://mempool.space/docs/api/rest#get-block)
//
//!
//! #### [GET /api/block/:hash](https://mempool.space/api/block/000000000000000015dc777b3ff2611091336355d3f0ee9766a2cf3be8e4b1ce)
//!
//!	mempool-space \--block 000000000000000015dc777b3ff2611091336355d3f0ee9766a2cf3be8e4b1ce
//!
//!	mempool-space_block 000000000000000015dc777b3ff2611091336355d3f0ee9766a2cf3be8e4b1ce
//!
//! #### [GET /api/block/:hash/header](https://mempool.space/api/block/0000000000000000000065bda8f8a88f2e1e00d9a6887a43d640e52a4c7660f2/header)
//!
//!	mempool-space \--block_header 0000000000000000000065bda8f8a88f2e1e00d9a6887a43d640e52a4c7660f2
//!
//!	mempool-space_block_header 0000000000000000000065bda8f8a88f2e1e00d9a6887a43d640e52a4c7660f2
//!
//! #### [GET /api/block-height:height](https://mempool.space/api/block-height/615615)
//!
//!	mempool-space \--block_height 615615
//!
//!	mempool-space_block_height 615615
//!
//! #### [GET /api/v1/mining/blocks/timestamp/:timestamp](https://mempool.space/api/v1/mining/blocks/timestamp/1672531200)
//!
//!	mempool-space \--blocks_timestamp 1672531200
//!
//!	mempool-space_blocks_timestamp 1672531200
//!
//! #### [GET /api/block/:hash/raw](https://mempool.space/api/block/0000000000000000000065bda8f8a88f2e1e00d9a6887a43d640e52a4c7660f2/raw)
//!
//!	mempool-space \--block_raw 0000000000000000000065bda8f8a88f2e1e00d9a6887a43d640e52a4c7660f2
//!
//!	mempool-space_block_raw 0000000000000000000065bda8f8a88f2e1e00d9a6887a43d640e52a4c7660f2
//!
//! #### [GET /api/block/:hash/status](https://mempool.space/api/block/0000000000000000000065bda8f8a88f2e1e00d9a6887a43d640e52a4c7660f2/status)
//!
//!	mempool-space \--block_status 0000000000000000000065bda8f8a88f2e1e00d9a6887a43d640e52a4c7660f2
//!
//!	mempool-space_block_status 0000000000000000000065bda8f8a88f2e1e00d9a6887a43d640e52a4c7660f2
//!
//! #### [GET /api/blocks/tip/height](https://mempool.space/api/blocks/tip/height)
//!
//!	mempool-space \--blocks_tip_height
//!
//!	mempool-space_blocks_tip_height
//!
//! #### [GET /api/blocks/tip/hash](https://mempool.space/api/blocks/tip/hash)
//!
//!	mempool-space \--blocks_tip_hash
//!
//!	mempool-space_blocks_tip_hash
//!
//! #### [GET /api/block/:hash/txid/:index](https://mempool.space/api/block/000000000000000015dc777b3ff2611091336355d3f0ee9766a2cf3be8e4b1ce/txid/218)
//!
//! mempool-space \--block_txid 000000000000000015dc777b3ff2611091336355d3f0ee9766a2cf3be8e4b1ce \--block_txindex 218
//!
//!	mempool-space_block_txid 000000000000000015dc777b3ff2611091336355d3f0ee9766a2cf3be8e4b1ce 218
//!
//! #### [GET /api/block/:hash/txids](https://mempool.space/api/block/000000000000000015dc777b3ff2611091336355d3f0ee9766a2cf3be8e4b1ce/txids)
//!
//! mempool-space \--block_txids 000000000000000015dc777b3ff2611091336355d3f0ee9766a2cf3be8e4b1ce
//!
//!	mempool-space_block_txids 000000000000000015dc777b3ff2611091336355d3f0ee9766a2cf3be8e4b1ce
//!
//! #### [GET /api/block/:hash/txs/:start_index](https://mempool.space/api/block/000000000000000015dc777b3ff2611091336355d3f0ee9766a2cf3be8e4b1ce/txs/0) (start_index % 25 == 0)
//!
//! mempool-space \--block_txs 000000000000000015dc777b3ff2611091336355d3f0ee9766a2cf3be8e4b1ce \--start_index 0
//!
//! mempool-space \--block_txs 000000000000000015dc777b3ff2611091336355d3f0ee9766a2cf3be8e4b1ce \--start_index 25
//!
//!	mempool-space_block_txs 000000000000000015dc777b3ff2611091336355d3f0ee9766a2cf3be8e4b1ce 0
//!
//!	mempool-space_block_txs 000000000000000015dc777b3ff2611091336355d3f0ee9766a2cf3be8e4b1ce 25
//!
//! #### [GET /api/v1/blocks[/:startHeight]](https://mempool.space/api/v1/blocks/730000)
//!
//! mempool-space \--blocks 730000
//!
//!	mempool-space_blocks 730000
//!
//! #### [GET /api/v1/blocks-bulk/:minHeight[/:maxHeight]](https://mempool.space/api/v1/blocks-bulk/100000/100000)
//!
//! mempool-space \--blocks_bulk \--min_height 730000 \--max_height 840000
//!
//!	mempool-space_blocks_bulk 730000 840000

#![warn(missing_docs, clippy::unwrap_used)]

///
/// A CLI tool for [`mempool_space`].
///
/// [`mempool_space`]: https://github.com/randymcmillan/mempool_space

/// TESTING
///
/// TESTING 2
///
/// TESTING 2
///
/// TESTING 2
///
/// TESTING 2

#[warn(missing_docs, clippy::unwrap_used)]
pub mod api;
pub mod blockheight;
pub mod blocking;
pub mod error;
pub mod resolve_policy;
pub mod target;

pub use error::{CheckTargetError, ParseTargetError, ResolveTargetError};
pub use resolve_policy::ResolvePolicy;
pub use target::{Fqhn, IcmpTarget, Port, Status, Target, TcpTarget};

#[cfg(feature = "async")]
pub use async_target::{AsyncTarget, AsyncTargetExecutor, BoxedHandler, BoxedTarget, OldStatus};

/// Command-line argument parser.
pub mod args;
#[cfg(feature = "async")]
pub mod async_target;

/// Configuration file parser.
pub mod config;
/// Custom error implementation.
pub mod this_error;
/// Upload handler.
pub mod upload;

use crate::args::Args;
use crate::config::Config;
use crate::this_error::{Error, Result};
use crate::upload::Uploader;
// use colored::Colorize;
use crossterm::style::Stylize;
use std::fs;
use std::io::IsTerminal;
use std::io::Read;
use std::io::{self}; //, Read};

use crate::blocking::blocking;

const URL: &str = "https://mempool.space/api";

// BOOM
//
/// mempool_space - a mempool.space API lib
///
/// Author: @RandyMcMillan (randy.lee.mcmillan@gmail.com)
///
/// <https://github.com/RandyMcMillan/mempool_space.git>

///
/// USAGE:
/// - mempool-space \--difficulty_adjustment (flagged)
/// - mempool-space_difficulty_adjustment (executable)
///   1. Flags follow the mempool.space api/rest (replace dashes with underscores)
///   2. Flags invoke the executable

///

/// mempool_space - a mempool.space API lib
///
/// Author: @RandyMcMillan (randy.lee.mcmillan@gmail.com)
///
/// <https://github.com/RandyMcMillan/mempool_space.git>

///
/// USAGE:
/// - mempool-space \--difficulty_adjustment (flagged)
/// - mempool-space_difficulty_adjustment (executable)
///   1. Flags follow the mempool.space api/rest (replace dashes with underscores)
///   2. Flags invoke the executable

/// mempool-space \--option_str
///
/// mempool-space \--blocks_tip_height
///
/// mempool-space \--blocks_tip_hash
///
/// mempool-space \--option_str arg
///
/// mempool-space \--block 00000000000000000002d5c6e3451f94af29a3606e42b3c8f1db3cdfdc2bc934
///
/// mempool-space \--block $(mempool-space \--blocks_tip_hash)
///
/// mempool-space_block $(mempool-space_blocks_tip_hash)
///

/// Default name of the configuration file.
const CONFIG_FILE: &str = "config.toml";

///  `pub fn run(args: Args) -> Result<()>`
pub fn run(args: Args) -> Result<()> {
    let mut config = Config::default();
    if let Some(ref config_path) = args.config {
        config = toml::from_str(&fs::read_to_string(config_path)?)?
    } else {
        for path in [
            dirs_next::home_dir().map(|p| p.join(".mempool").join(CONFIG_FILE)),
            dirs_next::config_dir().map(|p| p.join("rustypaste").join(CONFIG_FILE)),
        ]
        .iter()
        .filter_map(|v| v.as_ref())
        {
            if path.exists() {
                config = toml::from_str(&fs::read_to_string(path)?)?;
                break;
            }
        }
    }
    config.update_from_args(&args);
    if config.server.address.is_empty() {
        return Err(Error::NoServerAddressError);
    }

    let uploader = Uploader::new(&config);
    if args.print_server_version {
        println!("rustypaste-server {}", uploader.retrieve_version()?.trim());
        return Ok(());
    }

    if args.list_files {
        let prettify = args.prettify || config.style.as_ref().map(|style| style.prettify).unwrap_or(false);
        uploader.retrieve_list(&mut io::stdout(), prettify)?;
        return Ok(());
    }

    let mut results = Vec::new();
    if let Some(ref url) = args.url {
        results.push(uploader.upload_url(url));
    } else if let Some(ref remote_url) = args.remote {
        results.push(uploader.upload_remote_url(remote_url));
    } else if !std::io::stdin().is_terminal() || args.files.contains(&String::from("-")) {
        let mut buffer = Vec::new();
        let mut stdin = io::stdin();
        stdin.read_to_end(&mut buffer)?;
        results.push(uploader.upload_stream(&*buffer));
    } else {
        for file in args.files.iter() {
            if !args.delete {
                results.push(uploader.upload_file(file))
            } else {
                results.push(uploader.delete_file(file))
            }
        }
    }
    let prettify = args.prettify || config.style.as_ref().map(|style| style.prettify).unwrap_or(false);
    let format_padding = prettify
        .then(|| results.iter().map(|v| v.0.len()).max())
        .flatten()
        .unwrap_or(1);
    for (data, result) in results.iter().map(|v| (v.0, v.1.as_ref())) {
        let data = if prettify {
            format!(
                "{:p$} {} ",
                data,
                if result.is_ok() {
                    "=>".green().bold()
                } else {
                    "=>".red().bold()
                },
                p = format_padding,
            )
        } else {
            String::new()
        };
        match result {
            Ok(url) => println!("{}{}", data, url.trim()),
            Err(e) => eprintln!("{data}{e}"),
        }
    }

    Ok(())
}

// pub fn add(a: i32, b: i32) -> i32 {
//     a + b
// }
//
// // This is a really bad adding function, its purpose is to fail in this
// // example.
// #[allow(dead_code)]
// fn bad_add(a: i32, b: i32) -> i32 {
//     a - b
// }

/// Be nice to mempool.space when testing
pub fn wait(sleep: &str) {
    use std::process::Command;
    let sleep_cmd = Command::new("sleep").arg(sleep).output().expect("wait:sleep failed");
    let result = String::from_utf8(sleep_cmd.stdout)
        .map_err(|non_utf8| String::from_utf8_lossy(non_utf8.as_bytes()).into_owned())
        .unwrap();
    println!("\nwait:sleep:{:?}:{}", sleep, result);
    // if result.unwrap().len() <= 0 {
    //     true
    // } else {
    //     false
    // }
}

/// TESTS
#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use crate::api::api;

    /// General
    /// https://mempool.space/docs/api/rest
    /// cargo test -- --nocapture
    #[test]
    fn test_difficulty_adjustment() {
        // GET /api/v1/difficulty-adjustment
        let binding = format!("v1/difficulty-adjustment").clone();
        let difficulty_adjustment: &str = blocking(&binding).expect("REASON");
        let difficulty_adjustment = api("difficulty_adjustment", "extraneous_arg");
        wait("1");
    }
    #[test]
    fn test_price() {
        // GET /api/v1/prices
        let binding = format!("v1/prices").clone();
        let prices: &str = blocking(&binding).expect("REASON");
        let prices = api("prices", "extraneous_arg");
        wait("1");
    }
    #[test]
    fn test_historical_price() {
        use crate::args::historical_price;
        // GET /api/v1/historical-price?currency=EUR&timestamp=1500000000
        let historical_price_json = historical_price(&"EUR", &"1500000000");
        print!("\n{{\"prices\":[{{\"time\":1499904000,\"EUR\":1964,\"USD\":2254.9}}],\"exchangeRates\":{{\"USDEUR\":0.92,\"USDGBP\":0.78,\"USDCAD\":1.38,\"USDCHF\":0.87,\"USDAUD\":1.53,\"USDJPY\":146.62}}}}\n");
        let historical_prices = api("historical_price", "USD");
        wait("1");
    }

    /// Addresses
    #[test]
    fn test_address() {
        // GET /api/address/:address
        let binding = format!("address/1wiz18xYmhRX6xStj2b9t1rwWX4GKUgpv").clone();
        let address: &str = blocking(&binding).expect("test_address failed");
        let address = api("address", "1wiz18xYmhRX6xStj2b9t1rwWX4GKUgpv");
        wait("1");
    }
    #[test]
    fn test_address_txs() {
        // GET /api/address/:address/txs
        let binding = format!("address/1wiz18xYmhRX6xStj2b9t1rwWX4GKUgpv/txs").clone();
        let address_txs: &str = blocking(&binding).expect("test_address_txs failed");
        let address_txs = api("address_txs", "1wiz18xYmhRX6xStj2b9t1rwWX4GKUgpv");
        wait("1");
    }
    #[test]
    fn test_address_txs_chain() {
        // GET /api/address/:address/txs/chain
        let binding = format!("address/1wiz18xYmhRX6xStj2b9t1rwWX4GKUgpv/txs/chain").clone();
        let address_txs_chain: &str = blocking(&binding).expect("REASON");
        let address_txs_chain = api("address_txs_chain", "1wiz18xYmhRX6xStj2b9t1rwWX4GKUgpv");
        wait("1");
    }
    #[test]
    fn test_address_txs_mempool() {
        // GET /api/address/:address/txs/mempool
        let binding = format!("address/1wiz18xYmhRX6xStj2b9t1rwWX4GKUgpv/txs/mempool").clone();
        let address_txs_mempool: &str = blocking(&binding).expect("REASON");
        let address_txs_mempool = api("address_txs_mempool", "1wiz18xYmhRX6xStj2b9t1rwWX4GKUgpv");
        wait("1");
    }
    #[test]
    fn test_address_txs_utxo() {
        // GET /api/address/:address/utxo
        let binding = format!("address/1KFHE7w8BhaENAswwryaoccDb6qcT6DbYY/utxo").clone();
        let address_utxo: &str = blocking(&binding).expect("existing valid address needed");
        wait("1");
    }
    #[test]
    fn test_validate_address() {
        // GET /api/v1/validate-address/:address
        let binding = format!("v1/validate-address/1KFHE7w8BhaENAswwryaoccDb6qcT6DbYY").clone();
        let prices: &str = blocking(&binding).expect("valid address needed");
        wait("1");
    }

    /// Blocks
    #[test]
    fn test_block() {
        // GET /api/block/:hash
        let binding = format!("block/000000000000000015dc777b3ff2611091336355d3f0ee9766a2cf3be8e4b1ce").clone();
        let block: &str = blocking(&binding).expect("an existing block hash is needed");
        let block = api(
            "block",
            "000000000000000015dc777b3ff2611091336355d3f0ee9766a2cf3be8e4b1ce",
        );
        wait("1");
    }
    #[test]
    fn test_block_header() {
        // GET /api/block/:hash/header
        let binding = format!("block/000000000000000015dc777b3ff2611091336355d3f0ee9766a2cf3be8e4b1ce").clone();
        let block_header: &str = blocking(&binding).expect("an existing block hash is needed");
        let block = api(
            "block_header",
            "000000000000000015dc777b3ff2611091336355d3f0ee9766a2cf3be8e4b1ce",
        );
        wait("1");
    }
    #[test]
    fn test_block_height() {
        // GET /api/block-height:height
        let binding = format!("block-height/615615").clone();
        let block_height: &str = blocking(&binding).expect("an existing block hash is needed");
        let block_height = api("block_height", "615615");
        wait("1");
    }
    #[test]
    fn test_blocks_timestamp() {
        // GET /api/v1/mining/blocks/timestamp/:timestamp
        let binding = format!("v1/mining/blocks/timestamp/1672531200").clone();
        let timestamp: &str = blocking(&binding).expect("an existing block hash is needed");
        let timestamp = api("blocks_timestamp", "1672531200");
        wait("1");
    }
    #[test]
    fn test_block_raw() {
        // GET /api/block/:hash/raw
        let binding = format!("block/0000000000000000000065bda8f8a88f2e1e00d9a6887a43d640e52a4c7660f2").clone();
        let block_raw: &str = blocking(&binding).expect("an existing block hash is needed");
        let block_raw = api(
            "block_raw",
            "0000000000000000000065bda8f8a88f2e1e00d9a6887a43d640e52a4c7660f2",
        );
        wait("1");
    }

    #[test]
    fn test_block_status() {
        // GET /api/block/:hash/status
        let binding = format!("block/0000000000000000000065bda8f8a88f2e1e00d9a6887a43d640e52a4c7660f2").clone();
        let block_raw: &str = blocking(&binding).expect("an existing block hash is needed");
        let block_raw = api(
            "block_status",
            "0000000000000000000065bda8f8a88f2e1e00d9a6887a43d640e52a4c7660f2",
        );
        wait("1");
    }
    #[test]
    fn test_blocks_tip_height() {
        // GET /api/blocks/tip/height
        let binding = format!("blocks/tip/height").clone();
        let block_raw: &str = blocking(&binding).expect("returns current block_height");
        let block_raw = api("blocks_tip_height", "extraneous_arg");
        wait("1");
    }
    #[test]
    fn test_blocks_tip_hash() {
        // GET /api/blocks/tip/hash
        let binding = format!("blocks/tip/hash").clone();
        let block_raw: &str = blocking(&binding).expect("returns current block/tip/hash");
        let block_raw = api("blocks_tip_hash", "extraneous_arg");
        wait("1");
    }
    #[test]
    fn test_block_txid() {
        // GET /api/block/:hash/txid/:index
        let binding =
            format!("block/000000000000000015dc777b3ff2611091336355d3f0ee9766a2cf3be8e4b1ce/txid/218").clone();
        let get_block_txid: &str = blocking(&binding).expect("returns current txid from block index");
        let get_block_txid = api(
            "block_txid",
            "000000000000000015dc777b3ff2611091336355d3f0ee9766a2cf3be8e4b1ce/txid/218",
        );
        use crate::args::block_txid;
        block_txid(
            &"000000000000000015dc777b3ff2611091336355d3f0ee9766a2cf3be8e4b1ce",
            &"218",
        );
        wait("1");
    }
    #[test]
    fn test_block_txids() {
        // GET /api/block/:hash/txids
        let binding = format!("block/000000000000000015dc777b3ff2611091336355d3f0ee9766a2cf3be8e4b1ce/txids").clone();
        let get_block_txid: &str = blocking(&binding).expect("returns current txids from block");
        let get_block_txids = api(
            "block_txid",
            "000000000000000015dc777b3ff2611091336355d3f0ee9766a2cf3be8e4b1ce/txids",
        );
        use crate::args::block_txids;
        block_txids(&"000000000000000015dc777b3ff2611091336355d3f0ee9766a2cf3be8e4b1ce");
        wait("1");
    }
    #[test]
    fn test_block_txs() {
        // GET /api/block/:hash/txs[/:start_index] (start_index % 25 = 0)
        // )
        let binding = format!("block/000000000000000015dc777b3ff2611091336355d3f0ee9766a2cf3be8e4b1ce/txs").clone();
        let block_txs: &str = blocking(&binding).expect("returns current txids from block");
        let get_block_txs = api(
            "block_txs",
            "000000000000000015dc777b3ff2611091336355d3f0ee9766a2cf3be8e4b1ce/txs",
        );
        let get_block_txs = api(
            "block_txs",
            "000000000000000015dc777b3ff2611091336355d3f0ee9766a2cf3be8e4b1ce/txs/0",
        );
        let get_block_txs = api(
            "block_txs",
            "000000000000000015dc777b3ff2611091336355d3f0ee9766a2cf3be8e4b1ce/txs/1", //test if start_index_int % 25 == 0
        );
        let get_block_txs = api(
            "block_txs",
            "000000000000000015dc777b3ff2611091336355d3f0ee9766a2cf3be8e4b1ce/txs/25",
        );
    }
    #[test]
    fn test_blocks() {
        // GET /api/v1/blocks[/:startHeight]
        let binding = format!("v1/blocks/730000").clone();
        let get_block_txid: &str = blocking(&binding).expect("returns current txid from block index");
        let get_block_txid = api("blocks", "730000");
        let blocks_tip_height = api("blocks_tip_height", "extraneous_arg");
        use crate::args::blocks;
        blocks(&"");
        wait("1");
        blocks(&"0");
        wait("1");
        blocks(&"25");
        wait("1");
        blocks(&"730000");
        wait("1");
        blocks(&blocks_tip_height);
        wait("1");
    }
    #[test]
    //#[should_panic(expected = "This API is disabled. Set config.MEMPOOL.MAX_BLOCKS_BULK_QUERY to a positive number to enable it.
    #[should_panic]
    fn test_blocks_bulk() {
        // GET /api/v1/blocks-bulk/:minHeight[/:maxHeight]
        let binding = format!("v1/blocks-bulk/730000/840000").clone();
        let get_block_txid: &str = blocking(&binding).expect("returns current txid from block index");

        let get_block_txid = api("blocks_bulk", "730000/840000");
        let blocks_tip_height = api("blocks_tip_height", "extraneous_arg");
        use crate::args::blocks_bulk;
        blocks_bulk(&"0", &"0");
        blocks_bulk(&"0", &"1");
        blocks_bulk(&"730000", &"840000");
        blocks_bulk(&"730000", &blocks_tip_height);
        wait("1");
    }
    #[test]
    fn test_blockheight() {
        let blockheight = blockheight::blockheight();
        assert_ne!(0 as f64, blockheight.unwrap());
        wait("1");
    }
    /// Mining
    /// Fees
    /// Mempool
    /// Transactions
    /// Lightning
    /// Accelerator (Public)
    /// Accelerator (Authenticated)

    #[test]
    fn test_add() {
        // assert_eq!(add(1, 2), 3);
        wait("1");
    }

    #[test]
    fn test_bad_add() {
        // This assert would fire and test will fail.
        // Please note, that private functions can be tested too!
        // assert_ne!(bad_add(1, 2), 3);
        wait("1");
    }
    use std::panic::{catch_unwind, AssertUnwindSafe};
    #[test]
    fn should_panic() {
        let msg = catch_unwind(AssertUnwindSafe(|| {
            panic!(" foo panic message");
        }));

        assert_ne!("foo panic message", *msg.unwrap_err().downcast_ref::<&str>().unwrap());
        wait("1");
    }
}
