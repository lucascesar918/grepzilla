use std::env;
use std::process;

use grepzilla::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|_err| {
        process::exit(1);
    });

    if let Err(_e) = grepzilla::run(config) {
        process::exit(1);
    }
}
