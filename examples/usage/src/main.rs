// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// Author: Simon Brummer (simon.brummer@posteo.de)

use std::str::FromStr;

use mempool_space::*;

fn main() {
    // Construct ICMP Target check if the target is availabile
    let icmp_target = IcmpTarget::from_str("mempool.space").unwrap();
    match icmp_target.check_availability() {
        Ok(status) => println!("{} is {}", icmp_target.get_id(), status),
        Err(error) => println!("Check failed for {} reason {}", icmp_target.get_id(), error),
    }
}
