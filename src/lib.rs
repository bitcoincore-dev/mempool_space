// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// Author: Simon Brummer (simon.brummer@posteo.de)

//! Reachable, check if a Target is currently available or not.
//!
//! A "Target" is everything that implements the Target trait, used to
//! check if, a resource is currently available. This crate offers a ICMP and TCP based Target
//! usable to check, if a computer is available over the network.
//!
//! Additionally this crate contains asynchronous utilities to execute these checks regularly
//! within a given time interval.

use std::io::Read;

const VERSION: &str = "v1";
const URL: &str = "https://mempool.space/api";

pub fn blocking(api: &String) -> Result<&str> {
    let call = format!("{}/{}", URL, api);
    let mut body = ureq::get(&call).call().expect("REASON").into_reader();
    let mut buf = Vec::new();
    body.read_to_end(&mut buf).unwrap();
    let text = match std::str::from_utf8(&buf) {
        Ok(s) => s,
        Err(_) => panic!("Invalid ASCII data"),
    };
    print!("{}", text);
    Ok(api)
}

// Modules
pub mod blockheight;
pub mod error;
pub mod resolve_policy;
pub mod target;

#[cfg(feature = "async")]
pub mod async_target;

// Re-exports
pub use error::{CheckTargetError, ParseTargetError, ResolveTargetError};
pub use resolve_policy::ResolvePolicy;
pub use target::{Fqhn, IcmpTarget, Port, Status, Target, TcpTarget};

#[cfg(feature = "async")]
pub use async_target::{AsyncTarget, AsyncTargetExecutor, BoxedHandler, BoxedTarget, OldStatus};

// //! A CLI tool for [`rustypaste`].
// //!
// //! [`rustypaste`]: https://github.com/orhun/rustypaste
// #![warn(missing_docs, clippy::unwrap_used)]

/// Command-line argument parser.
pub mod args;
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
use std::fs;
use std::io::IsTerminal;
use std::io::{self}; //, Read};

use crossterm::style::Stylize;

/// Default name of the configuration file.
const CONFIG_FILE: &str = "config.toml";

/// Runs `rpaste`.
pub fn run(args: Args) -> Result<()> {
    let mut config = Config::default();
    if let Some(ref config_path) = args.config {
        config = toml::from_str(&fs::read_to_string(config_path)?)?
    } else {
        for path in [
            dirs_next::home_dir().map(|p| p.join(".rustypaste").join(CONFIG_FILE)),
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
