/// Entrypoint for modexp command line utility
///
/// Alexander DuPree 2021
use std::env;

use modexp::modexp;

/// Print a usage error message and exit
fn error() -> ! {
    eprintln!("modexp: usage: modexp <x> <y> <m>");
    std::process::exit(1);
}

/// modexp command line utility entry point
fn main() {
    let args: Vec<u32> = env::args()
        .skip(1)
        .map(|s| s.parse().unwrap_or_else(|_| error()))
        .collect();

    if args.len() != 3 {
        error();
    }

    println!("{}", modexp(args[0], args[1], args[2]));

    std::process::exit(0);
}
