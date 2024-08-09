use mempool_space::args::Args;
use std::process;

pub fn main() {
    let args = Args::parse();
    match mempool_space::run(args) {
        Ok(_) => process::exit(0),
        Err(e) => {
            eprintln!("{e}");
            process::exit(1)
        }
    }
}
