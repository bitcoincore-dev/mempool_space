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


use std::time::SystemTime;
use std::io::Read;
use reqwest::Url;

use crate::blockheight::blockheight;

const URL: &str = "https://mempool.space/api/blocks/tip/height";

// Modules
pub mod blockheight;
pub mod error;
pub mod resolve_policy;
pub mod target;
pub fn get_blockheight() -> Result<String, &'static str> {
    let _blockheight_no_nl = blockheight().unwrap().to_string();

    Ok(format!("{}", blockheight().unwrap().to_string()))
}
async fn non_blocking(n: usize) -> usize {
    let tasks = (0..n)
        .into_iter()
        .map(|_| {
            tokio::spawn(async move {
                let since_the_epoch = SystemTime::now()
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .expect("get millis error");
                let seconds = since_the_epoch.as_secs();
                let subsec_millis = since_the_epoch.subsec_millis() as u64;
                let _now_millis = seconds * 1000 + subsec_millis;
                // println!("_now_millis: {}", seconds * 1000 + subsec_millis);

                let _ = get_blockheight();
                let url = Url::parse(URL).unwrap();
                let mut res = reqwest::blocking::get(url).unwrap();

                let mut tmp_string = String::new();
                res.read_to_string(&mut tmp_string).unwrap();
                // println!("{}", format!("{:?}", res));
                let tmp_u64 = tmp_string.parse::<u64>().unwrap_or(0);
                println!("{}", format!("{:?}", tmp_u64));

                // TODO:impl gnostr-weeble_millis
                // let weeble = now_millis as f64 / tmp_u64 as f64;
                // let weeble = seconds as f64 / tmp_u64 as f64;
                // println!("{}", format!("{}", weeble.floor()));

                let body = reqwest::get(URL).await.unwrap().bytes();
                body.await.unwrap().len()
                // print block count from mempool.space or panic
                // let text = match std::str::from_utf8(&body) {
                //    Ok(s) => s,
                //    Err(_) => panic!("Invalid ASCII data"),
                //};
                // println!("{}", text);
            })
        })
        .collect::<Vec<_>>();

    let mut res = 0;
    for task in tasks {
        res += task.await.unwrap();
    }
    res
}
fn blocking(n: usize) -> usize {
    (0..n)
        .into_iter()
        .map(|_| {
            std::thread::spawn(|| {
                let mut body = ureq::get(URL).call().expect("REASON").into_reader();
                let mut buf = Vec::new();
                body.read_to_end(&mut buf).unwrap();
                // print block count from mempool.space or panic
                let text = match std::str::from_utf8(&buf) {
                    Ok(s) => s,
                    Err(_) => panic!("Invalid ASCII data"),
                };
                println!("{}", text);
                buf.len()
            })
        })
        .collect::<Vec<_>>()
        .into_iter()
        .map(|it| it.join().unwrap())
        .sum()
}

#[cfg(feature = "async")]
pub mod async_target;

// Re-exports
pub use error::{CheckTargetError, ParseTargetError, ResolveTargetError};
pub use resolve_policy::ResolvePolicy;
pub use target::{Fqhn, IcmpTarget, Port, Status, Target, TcpTarget};

#[cfg(feature = "async")]
pub use async_target::{AsyncTarget, AsyncTargetExecutor, BoxedHandler, BoxedTarget, OldStatus};
