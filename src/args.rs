use getopts::Options;
use std::env;
use std::io::IsTerminal;
use std::path::PathBuf;
use std::process;

use crate::blocking;

/// mempool-space_ARG
/// mempool-space --arg
/// mempool-space_ARG_STRING
/// mempool-space --arg_string
pub fn generic_sys_call(option: &str, sub_string: &str) -> String {
    use std::process::Command;

    //if sub_string == "v1" {
    //    print!("TODO: support --version v1 api versioning.");
    //} else if sub_string == "v2" {
    //    print!("TODO: support --version v2 api versioning.");
    //} else {
    let output = if cfg!(target_os = "windows") {
        Command::new(format!("mempool-space_{}", option))
            .args(["/C", sub_string])
            .output()
            .expect("failed to execute process")
    } else {
        Command::new(format!("mempool-space_{}", option))
            .arg(sub_string)
            //.arg("echo hello")
            .output()
            .expect("failed to execute process")
    };

    let result = String::from_utf8(output.stdout)
        .map_err(|non_utf8| String::from_utf8_lossy(non_utf8.as_bytes()).into_owned())
        .unwrap();
    print!("{}", result);
    result
    //}
}
/// GET /api/v1/historical-price?currency=CURRENCY&timestamp=TIMESTAMP
/// <https://mempool.space/docs/api/rest#get-historical-price>
pub fn historical_price(currency: &str, timestamp: &str) {
    let _res = blocking(&format!(
        "v1/historical-price?currency={}&timestamp={}",
        &format!("{:}", &currency),
        &format!("{:}", &timestamp)
    ));
}
/// GET /api/block/:hash/txid/:index
/// <https://mempool.space/docs/api/rest#get-block-transaction-id>
pub fn block_txid(block_hash: &str, txindex: &str) {
    let _res = blocking(&format!("block/{}/txid/{}", block_hash, txindex));
}
/// GET /api/block/:hash/txids
/// <https://mempool.space/docs/api/rest#get-block-transaction-ids>
pub fn block_txids(block_hash: &str) {
    let _res = blocking(&format!("block/{}/txids", block_hash));
}
/// GET /api/block/:hash/txs[/:start_index] (start_index % 25 = 0)
/// <https://mempool.space/docs/api/rest#get-block-transactions>
pub fn block_txs(block_hash: &str, start_index: &str) {
    let start_index_int = start_index.parse::<i32>().unwrap_or(0);
    if start_index_int % 25 == 0 {
        let _res = blocking(&format!("block/{}/txs/{}", block_hash, start_index));
    } else {
        let _res = blocking(&format!("block/{}/txs/{}", block_hash, &"0"));
    }
}
/// GET /api/v1/blocks[/:startHeight]
/// <https://mempool.space/docs/api/rest#get-blocks>
pub fn blocks(start_height: &str) {
    //TODO blocks_tip_height
    let start_height_int = start_height.parse::<i32>().unwrap_or(0);
    if start_height_int >= 0 {
        let _res = blocking(&format!("v1/blocks/{}", start_height));
    } else {
        let _res = blocking(&format!("v1/blocks"));
    }
}

/// <https://mempool.space/docs/api/rest>
/// - [API/REST](https://mempool.space/docs/api/rest)
///     - [GENERAL](https://mempool.space/docs/api/rest#get-difficulty-adjustment)
///     - [ADDRESSES](https://mempool.space/docs/api/rest#get-address)
///     - [BLOCKS](https://mempool.space/docs/api/rest#get-block)
///     - [MINING](https://mempool.space/docs/api/rest#get-mining-pools)
///     - [FEES](https://mempool.space/docs/api/rest#get-mempool-blocks-fees)
///     - [MEMPOOL](https://mempool.space/docs/api/rest#get-mempool)
///     - [TRANSACTIONS](https://mempool.space/docs/api/rest#get-cpfp)
///     - [LIGHTNING](https://mempool.space/docs/api/rest#get-lightning-network-stats)
///     - [ACCELERATOR (Public)](https://mempool.space/docs/api/rest#accelerator-estimate)
///     - [ACCELERATOR (Authenticated)](https://mempool.space/docs/api/rest#accelerator-top-up-history)
///
///         USAGE:
///       - mempool-space \--difficulty_adjustment (flagged)
///       - mempool-space_difficulty_adjustment (executable)
///         1. Flags follow the mempool.space api/rest (replace dashes with underscores)
///         2. Flags invoke the executable
///
#[derive(Debug, Default)]
pub struct Args {
    // VERSION
    // pub version: Option<String>,
    /// `https://mempool.space/api/v1/difficulty-adjustment`
    pub difficulty_adjustment: Option<String>,
    /// `https://mempool.space/api/v1/prices`
    pub prices: Option<String>,
    /// `https://mempool.space/api/v1/historical-price`
    pub historical_price: Option<String>,
    /// `https://mempool.space/api/v1/historical-price?currency=USD`
    pub currency: Option<String>,
    /// `https://mempool.space/api/v1/historical-price?currency=USD?timestamp=0`
    pub timestamp: Option<String>,

    /// - ADDRESSES
    /// `https://mempool.space/api/address/<ADDRESS>`
    pub address: Option<String>,
    /// `https://mempool.space/api/address/<ADDRESS>/txs`
    pub address_txs: Option<String>,
    /// `https://mempool.space/api/address/<ADDRESS>/txs/chain`
    pub address_txs_chain: Option<String>,
    /// `https://mempool.space/address/<ADDRESS>/txs/mempool`
    pub address_txs_mempool: Option<String>,
    /// `https://mempool.space/api/address/<ADDRESS>/utxo`
    pub address_utxos: Option<String>,
    /// `https://mempool.space/api/validate-address/<ADDRESS>`
    pub validate_address: Option<String>,

    /// - BLOCK
    /// `https://mempool.space/api/block/<BLOCK_HASH>`
    pub block: Option<String>,

    /// - BLOCK_HEADER
    /// `https://mempool.space/api/block/<BLOCK_HASH>/header`
    pub block_header: Option<String>,

    /// - BLOCK_HEIGHT
    /// `https://mempool.space/api/block-height/<BLOCK_HEIGHT>`
    pub block_height: Option<String>,

    /// - V1 MINING BLOCKS TIMESTAMP <UTC_SECS>
    /// `https://mempool.space/api/v1/mining/blocks/timestamp/<UTC_SECS>`
    pub blocks_timestamp: Option<String>,

    /// - BLOCK
    /// `https://mempool.space/api/block/<BLOCK_HASH>/raw`
    pub block_raw: Option<String>,
    /// `https://mempool.space/api/block/<BLOCK_HASH>/status`
    pub block_status: Option<String>,

    /// - BLOCKS TIP HEIGHT
    /// `https://mempool.space/api/blocks/tip/height`
    pub blocks_tip_height: Option<String>,
    /// - BLOCKS TIP HASH
    /// `https://mempool.space/api/blocks/tip/hash`
    pub blocks_tip_hash: Option<String>,

    /// - BLOCK <BLOCK_HASH> <TXID> <INDEX>
    /// `https://mempool.space/api/block/<TXID>/<INDEX>`
    pub block_txid: Option<String>,
    pub block_txindex: Option<String>,

    /// - BLOCK <BLOCK_HASH> <TXIDS>
    /// `https://mempool.space/api/block/<TXID>`
    pub block_txids: Option<String>,

    /// - BLOCK <BLOCK_HASH> <TXS>
    /// `https://mempool.space/api/block/<BLOCK_HASH>/txs`
    pub block_txs: Option<String>,
    pub block_start_index: Option<String>,

    /// - V1 BLOCKS <BLOCK_HEIGHT>
    /// `https://mempool.space/api/v1/blocks/<BLOCKS_START_HEIGHT>`
    pub blocks: Option<String>,

    /// - V1 BLOCKS_BULK <BLOCK_HEIGHT_START> <BLOCK_HEIGHT_STOP>
    /// `https://mempool.space/api/v1/blocks-bulk/<BLOCK_HEIGHT_START>/<BLOCK_HEIGHT_STOP>`
    pub blocks_bulk: Option<String>,

    /// Configuration file.
    pub config: Option<PathBuf>,
    /// Server address.
    pub server: Option<String>,
    /// Authentication or delete token.
    pub auth: Option<String>,
    /// URL to shorten.
    pub url: Option<String>,
    /// Remote URL to download file.
    pub remote: Option<String>,
    /// Files to upload.
    pub files: Vec<String>,
    /// Whether if the file will disappear after being viewed once.
    pub oneshot: bool,
    /// Expiration time for the link.
    pub expire: Option<String>,
    /// Prettify the program output.
    pub prettify: bool,
    /// Whether if the server version should be printed.
    pub print_server_version: bool,
    /// List files on the server (file name, file size, expiry timestamp).
    pub list_files: bool,
    /// Delete files from server.
    pub delete: bool,
    /// Send filename header (give uploaded file a specific name).
    pub filename: Option<String>,
}

impl Args {
    /// Parses the command-line arguments.
    pub fn parse() -> Self {
        let mut opts = Options::new();

        //OPTFLAG
        opts.optflag("h", "help", "prints help information");
        opts.optflag("v", "vv", "prints version information");
        opts.optflag("V", "server-version", "retrieves the server version");
        opts.optflag("l", "list", "lists files on the server");
        opts.optflag("d", "delete", "delete files from server");
        opts.optflag("o", "oneshot", "generates one shot links");
        opts.optflag("p", "pretty", "prettifies the output");

        //mempool api intercepts
        // VERSION
        // premeptive support v1,v2 etc...
        // opts.optopt("", "version", "api call version path (v1/...)", "VERSION");
        // GENERAL
        opts.optflag("", "difficulty_adjustment", "difficulty_adjustment api call");
        opts.optflag("", "prices", "prices api call");
        opts.optflag("", "historical_price", "historical_price api call");
        opts.optopt("", "timestamp", "timestamp api call", "TIMESTAMP");
        opts.optopt("", "currency", "currency api call", "CURRENCY");

        // ADDRESSES
        opts.optopt("", "address", "address api call", "ADDRESS");
        opts.optopt("", "address_txs", "address_txs api call", "ADDRESS_TXS");
        opts.optopt(
            "",
            "address_txs_chain",
            "address_txs_chain api call",
            "ADDRESS_TXS_CHAIN",
        );
        opts.optopt(
            "",
            "address_txs_mempool",
            "address_txs_mempool api call",
            "ADDRESS_TXS_MEMPOOL",
        );
        opts.optopt("", "address_utxos", "address_utxos api call", "ADDRESS_UTXOS");
        opts.optopt("", "validate_address", "validate an address", "VALIDATE_ADDRESS");

        // BLOCK/S
        opts.optopt("", "block", "block api call", "BLOCK");
        opts.optopt("", "block_header", "block-header api call", "BLOCK_HEADER");
        opts.optopt("", "block_height", "block-height api call", "BLOCK_HEIGHT");
        opts.optopt("", "blocks_timestamp", "blocks-timestamp api call", "BLOCKS_TIMESTAMP");
        opts.optopt("", "block_raw", "block-raw api call", "BLOCK_RAW");
        opts.optopt("", "block_status", "block-status api call", "BLOCK_STATUS");

        opts.optflag("", "blocks_tip_height", "GET /api/blocks/tip/height api call");
        opts.optflag("", "blocks_tip_hash", "GET /api/blocks/tip/hash api call");

        opts.optopt("", "block_txid", "block txid api call", "BLOCK_TXID");
        opts.optopt("", "block_txindex", "block_txindex api call", "BLOCK_TXINDEX");
        opts.optopt("", "block_txids", "block txids api call", "BLOCK_TXIDS");

        opts.optopt("", "block_txs", "block txs api call", "BLOCK_TXS");
        opts.optopt("", "block_start_index", "block txs api call", "BLOCK_START_INDEX");

        opts.optopt("", "blocks", "block txids api call", "BLOCKS_START_HEIGHT");
        opts.optopt("", "blocks_bulk", "block txids api call", "BLOCKS_BULK");

        //OPTOPT
        opts.optopt("c", "config", "sets the configuration file", "CONFIG");
        opts.optopt("s", "server", "sets the address of the rustypaste server", "SERVER");
        opts.optopt("a", "auth", "sets the authentication or delete token", "TOKEN");
        opts.optopt("u", "url", "sets the URL to shorten", "URL");
        opts.optopt("r", "remote", "sets the remote URL for uploading", "URL");
        opts.optopt("e", "expire", "sets the expiration time for the link", "TIME");
        opts.optopt("n", "filename", "sets and overrides the filename", "NAME");

        let env_args: Vec<String> = env::args().collect();
        let matches = match opts.parse(&env_args[1..]) {
            Ok(m) => m,
            Err(e) => {
                eprintln!("Argument error: `{e}`");
                process::exit(1);
            }
        };

        //mempool api intercepts
        // VERSION
        // GENERAL
        if matches.opt_present("difficulty_adjustment") {
            generic_sys_call("difficulty_adjustment", &"v9999");
            std::process::exit(0);
        }
        if matches.opt_present("prices") {
            generic_sys_call("prices", &"v9999");
            std::process::exit(0);
        }
        if matches.opt_present("historical_price") {
            if matches.opt_present("currency") {
                //print!("currency={}\n", matches.opt_present("currency"));
                let currency = matches.opt_str("currency");
                //print!("currency={}", currency.clone().unwrap());
                if matches.opt_present("timestamp") {
                    //print!("timestamp={}\n", matches.opt_present("timestamp"));
                    let timestamp = matches.opt_str("timestamp");
                    historical_price(&currency.as_ref().unwrap(), &timestamp.unwrap());
                } else {
                    historical_price(&currency.unwrap(), "");
                }
            } else {
                historical_price("", "");
            }

            //historical_prices(&"USD", &"1500000000");
            //historical_prices(&"EUR", &"1500000000");
            std::process::exit(0);
        }

        // ADDRESSES
        if matches.opt_present("address") {
            let address = matches.opt_str("address");
            generic_sys_call("address", &address.unwrap());
            std::process::exit(0);
        }
        if matches.opt_present("address_txs") {
            let address = matches.opt_str("address_txs");
            generic_sys_call("address_txs", &address.unwrap());
            std::process::exit(0);
        }
        if matches.opt_present("address_txs_chain") {
            let address = matches.opt_str("address_txs_chain");
            generic_sys_call("address_txs_chain", &address.unwrap());
            std::process::exit(0);
        }
        if matches.opt_present("address_txs_mempool") {
            let address = matches.opt_str("address_txs_mempool");
            generic_sys_call("address_txs_mempool", &address.unwrap());
            std::process::exit(0);
        }
        if matches.opt_present("validate_address") {
            let validate_address = matches.opt_str("validate_address");
            generic_sys_call("validate_address", &validate_address.unwrap());
            std::process::exit(0);
        }

        if matches.opt_present("block") {
            let block = matches.opt_str("block");
            generic_sys_call("block", &block.unwrap());
            std::process::exit(0);
        }
        if matches.opt_present("block_header") {
            let block_header = matches.opt_str("block_header");
            generic_sys_call("block_header", &block_header.unwrap());
            std::process::exit(0);
        }
        if matches.opt_present("block_height") {
            let block_height = matches.opt_str("block_height");
            generic_sys_call("block_height", &block_height.unwrap());
            std::process::exit(0);
        }
        //blocks_timestamp
        if matches.opt_present("blocks_timestamp") {
            let blocks_timestamp = matches.opt_str("blocks_timestamp");
            generic_sys_call("blocks_timestamp", &blocks_timestamp.unwrap());
            std::process::exit(0);
        }
        if matches.opt_present("block_raw") {
            let block_raw = matches.opt_str("block_raw");
            generic_sys_call("block_raw", &block_raw.unwrap());
            std::process::exit(0);
        }
        if matches.opt_present("block_status") {
            let block_status = matches.opt_str("block_status");
            generic_sys_call("block_status", &block_status.unwrap());
            std::process::exit(0);
        }
        if matches.opt_present("blocks_tip_height") {
            generic_sys_call("blocks_tip_height", &"extraneous_arg");
            std::process::exit(0);
        }
        if matches.opt_present("blocks_tip_hash") {
            generic_sys_call("blocks_tip_hash", &"extraneous_arg");
            std::process::exit(0);
        }
        if matches.opt_present("block_txid") {
            let arg_block_txid = matches.opt_str("block_txid"); //expect a block_hash
            let arg_block_txindex = matches.opt_str("block_txindex");
            block_txid(&arg_block_txid.unwrap(), &arg_block_txindex.unwrap());
            std::process::exit(0);
        }
        if matches.opt_present("block_txids") {
            let arg_block_txids = matches.opt_str("block_txids"); //expect a block_hash
            block_txids(&arg_block_txids.unwrap());
            std::process::exit(0);
        }
        if matches.opt_present("block_txs") {
            let arg_block_txs = matches.opt_str("block_txs"); //expect a block_hash
            let arg_block_start_index = matches.opt_str("block_start_index");
            block_txs(&arg_block_txs.unwrap(), &arg_block_start_index.unwrap());
            std::process::exit(0);
        }
        if matches.opt_present("blocks") {
            let arg_blocks = matches.opt_str("blocks"); //expect a integer as string
            blocks(&arg_blocks.unwrap());
            std::process::exit(0);
        }

        if matches.opt_present("h")
            || (matches.free.is_empty()
                && !matches.opt_present("u")
                && !matches.opt_present("r")
                && !matches.opt_present("V")
                && !matches.opt_present("l")
                && !matches.opt_present("d")
                && !matches.opt_present("v")
                && std::io::stdin().is_terminal())
        {
            let usage = format!(
                "\n{} {} \u{2014} {}.\
                \n\u{221F} written by {}\
                \n\u{221F} licensed under MIT <{}>\
                \n\nUsage:\n    {} [options] <file(s)>",
                env!("CARGO_PKG_NAME"),
                env!("CARGO_PKG_VERSION"),
                env!("CARGO_PKG_DESCRIPTION"),
                env!("CARGO_PKG_AUTHORS"),
                env!("CARGO_PKG_REPOSITORY"),
                "mempool-space",
            );
            println!("{}", opts.usage(&usage));
            process::exit(0)
        }

        if matches.opt_present("v") {
            println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
            process::exit(0)
        }

        Args {
            config: env::var("RPASTE_CONFIG")
                .ok()
                .or_else(|| matches.opt_str("c"))
                .map(PathBuf::from),

            // mempool api intercepts
            // mempool api version
            // version: matches.opt_str("version"),

            // GENERAL
            difficulty_adjustment: matches.opt_str("difficulty_adjustment"),
            currency: matches.opt_str("currency"),
            prices: matches.opt_str("prices"),
            timestamp: matches.opt_str("timestamp"),
            historical_price: matches.opt_str("historical_price"),

            // ADDRESSES
            address: matches.opt_str("address"),
            address_txs: matches.opt_str("address_txs"),
            address_txs_chain: matches.opt_str("address_txs_chain"),
            address_txs_mempool: matches.opt_str("address_txs_mempool"),
            address_utxos: matches.opt_str("address_utxos"),
            validate_address: matches.opt_str("validate_address"),

            // BLOCK/S
            // https://mempool.space/api/block/<endpoint>
            // https://mempool.space/api/block/<block_hash>
            // BLOCK
            block: matches.opt_str("block"),
            // https://mempool.space/api/block/<block_hash>/header
            block_header: matches.opt_str("block_header"),
            // BLOCK_HEIGHT
            // https://mempool.space/api/block-height/615615
            block_height: matches.opt_str("block_height"),

            // V1 MINING BLOCKS TIMESTAMP
            // https://mempool.space/api/v1/mining/blocks/timestamp/<UTC_SECS>"
            blocks_timestamp: matches.opt_str("blocks_timestamp"),

            // BLOCK
            // https://mempool.space/api/block/<block_hash>/raw
            block_raw: matches.opt_str("block_raw"),
            // https://mempool.space/api/block/<block_hash>/status
            block_status: matches.opt_str("block_status"),

            // BLOCKS
            // BLOCKS TIP HEIGHT
            // https://mempool.space/api/blocks/tip/height
            blocks_tip_height: matches.opt_str("blocks_tip_height"),
            // BLOCKS TIP HASH
            // https://mempool.space/api/blocks/tip/hash
            blocks_tip_hash: matches.opt_str("blocks_tip_hash"),

            // BLOCK
            // BLOCK BLOCK_HASH TXID INDEX
            // https://mempool.space/api/block/<block_hash>/<txid>/<index>
            block_txid: matches.opt_str("block_txid"),
            block_txindex: matches.opt_str("block_txindex"),
            // BLOCK BLOCK_HASH TXIDS
            // https://mempool.space/api/block/<block_hash>/<txids>
            block_txids: matches.opt_str("block_txids"),
            // BLOCK BLOCK_HASH TXS
            // https://mempool.space/api/block/<block_hash>/<txs>
            block_txs: matches.opt_str("block_txs"),
            block_start_index: matches.opt_str("block_start_index"),

            // V1 BLOCKS
            // https://mempool.space/api/v1/blocks/<BLOCK_HEIGHT>"
            blocks: matches.opt_str("blocks"),

            // V1 BLOCKS_BULK
            // https://mempool.space/api/v1/blocks-bulk/<BLOCK_HEIGHT_START>/<BLOCK_HEIGHT_STOP>"
            blocks_bulk: matches.opt_str("blocks_bulk"),

            server: matches.opt_str("s"),
            auth: matches.opt_str("a"),
            url: matches.opt_str("u"),
            remote: matches.opt_str("r"),
            oneshot: matches.opt_present("o"),
            expire: matches.opt_str("e"),
            prettify: matches.opt_present("p"),
            print_server_version: matches.opt_present("V"),
            list_files: matches.opt_present("l"),
            delete: matches.opt_present("d"),
            filename: matches.opt_str("n"),
            files: matches.free,
        }
    }
}
