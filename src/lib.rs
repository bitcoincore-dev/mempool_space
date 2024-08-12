// Originally based on the reachable repo.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// Author: Simon Brummer (simon.brummer@posteo.de)

//! mempool_space, a mempool.space API lib.
//!
//! Author: @RandyMcMillan (randy.lee.mcmillan@gmail.com)
//! <https://github.com/RandyMcMillan/mempool_space.git>

use std::io::Read;

//const API_VERSION: &str = "v1";
//
const URL: &str = "https://mempool.space/api";

///  `pub fn blocking(api: &String) -> Result<&str>`
pub fn blocking(api: &String) -> Result<&str> {
    let call = format!("{}/{}", URL, api);
    let mut body = ureq::get(&call)
        .call()
        .expect("calls to blocking(api: &String) needs to include /v1/<api_endpoint> in some cases.")
        .into_reader();
    let mut buf = Vec::new();
    body.read_to_end(&mut buf).unwrap();
    let text = match std::str::from_utf8(&buf) {
        Ok(s) => s,
        Err(_) => panic!("Invalid ASCII data"),
    };
    print!("{}", text);
    Ok(api)
}

/// `pub mod blockheight`
pub mod blockheight;
/// `pub mod error`
pub mod error;
/// `pub mod resolve_policy`
pub mod resolve_policy;
/// `pub mod target`
pub mod target;

#[cfg(feature = "async")]
/// `pub mod async_target`
pub mod async_target;

// Re-exports
/// `pub use error`
pub use error::{CheckTargetError, ParseTargetError, ResolveTargetError};
/// `pub use resolve_policy`
pub use resolve_policy::ResolvePolicy;
/// `pub use target`
pub use target::{Fqhn, IcmpTarget, Port, Status, Target, TcpTarget};

#[cfg(feature = "async")]
pub use async_target::{AsyncTarget, AsyncTargetExecutor, BoxedHandler, BoxedTarget, OldStatus};

// //! A CLI tool for [`mempool_space`].
// //!
// //! [`mempool_space`]: https://github.com/randymcmillan/mempool_space
// #![warn(missing_docs, clippy::unwrap_used)]

/// Command-line argument parser.
pub mod args;
/// Configuration file parser.
pub mod config;
/// Custom error implementation.
pub mod this_error;
/// Upload handler.
pub mod upload;

use crate::args::{generic_sys_call, historical_price, Args};
use crate::config::Config;
use crate::this_error::{Error, Result};
use crate::upload::Uploader;
// use colored::Colorize;
use std::fs;
use std::io::IsTerminal;
use std::io::{self}; //, Read};

use crossterm::style::Stylize;

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

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    /// General
    /// https://mempool.space/docs/api/rest
    /// cargo test -- --nocapture
    #[test]
    fn test_difficulty_adjustment() {
        // GET /api/v1/difficulty-adjustment
        let binding = format!("v1/difficulty-adjustment").clone();
        let difficulty_adjustment: &str = blocking(&binding).expect("REASON");
        let difficulty_adjustment = generic_sys_call("difficulty_adjustment", "extraneous_arg");
        wait("1");
    }
    #[test]
    fn test_price() {
        // GET /api/v1/prices
        let binding = format!("v1/prices").clone();
        let prices: &str = blocking(&binding).expect("REASON");
        let prices = generic_sys_call("prices", "extraneous_arg");
        wait("1");
    }
    #[test]
    fn test_historical_price() {
        // GET /api/v1/historical-price?currency=EUR&timestamp=1500000000
        let historical_price_json = historical_price(&"EUR", &"1500000000");
        print!("\n{{\"prices\":[{{\"time\":1499904000,\"EUR\":1964,\"USD\":2254.9}}],\"exchangeRates\":{{\"USDEUR\":0.92,\"USDGBP\":0.78,\"USDCAD\":1.38,\"USDCHF\":0.87,\"USDAUD\":1.53,\"USDJPY\":146.62}}}}\n");
        let historical_prices = generic_sys_call("historical_price", "USD");
        wait("1");
    }

    /// Addresses
    #[test]
    fn test_address() {
        // GET /api/address/:address
        let binding = format!("address/1wiz18xYmhRX6xStj2b9t1rwWX4GKUgpv").clone();
        let address: &str = blocking(&binding).expect("test_address failed");
        let address = generic_sys_call("address", "1wiz18xYmhRX6xStj2b9t1rwWX4GKUgpv");
        wait("1");
    }
    #[test]
    fn test_address_txs() {
        // GET /api/address/:address/txs
        let binding = format!("address/1wiz18xYmhRX6xStj2b9t1rwWX4GKUgpv/txs").clone();
        let address_txs: &str = blocking(&binding).expect("test_address_txs failed");
        let address_txs = generic_sys_call("address_txs", "1wiz18xYmhRX6xStj2b9t1rwWX4GKUgpv");
        wait("1");
    }
    #[test]
    fn test_address_txs_chain() {
        // GET /api/address/:address/txs/chain
        let binding = format!("address/1wiz18xYmhRX6xStj2b9t1rwWX4GKUgpv/txs/chain").clone();
        let address_txs_chain: &str = blocking(&binding).expect("REASON");
        let address_txs_chain = generic_sys_call("address_txs_chain", "1wiz18xYmhRX6xStj2b9t1rwWX4GKUgpv");
        wait("1");
    }
    #[test]
    fn test_address_txs_mempool() {
        // GET /api/address/:address/txs/mempool
        let binding = format!("address/1wiz18xYmhRX6xStj2b9t1rwWX4GKUgpv/txs/mempool").clone();
        let address_txs_mempool: &str = blocking(&binding).expect("REASON");
        let address_txs_mempool = generic_sys_call("address_txs_mempool", "1wiz18xYmhRX6xStj2b9t1rwWX4GKUgpv");
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
        let block = generic_sys_call(
            "block",
            "000000000000000015dc777b3ff2611091336355d3f0ee9766a2cf3be8e4b1ce",
        );
        wait("1");
    }
    fn test_block_header() {
        // GET /api/block/:hash/header
        let binding = format!("block/000000000000000015dc777b3ff2611091336355d3f0ee9766a2cf3be8e4b1ce").clone();
        let block_header: &str = blocking(&binding).expect("an existing block hash is needed");
        let block = generic_sys_call(
            "block_header",
            "000000000000000015dc777b3ff2611091336355d3f0ee9766a2cf3be8e4b1ce",
        );
        wait("1");
    }
    fn test_block_height() {
        // GET /api/block-height:height
        let binding = format!("block-height/615615").clone();
        let block_height: &str = blocking(&binding).expect("an existing block hash is needed");
        let block_height= generic_sys_call(
            "block_height",
            "615615",
        );
        wait("1");
    }
    fn test_timestamp() {
        // GET /api/v1/mining/blocks/timestamp/:timestamp
        let binding = format!("v1/mining/blocks/timestamp/1672531200").clone();
        let timestamp: &str = blocking(&binding).expect("an existing block hash is needed");
        let timestamp= generic_sys_call(
            "timestamp",
            "1672531200",
        );
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
