use std::env;
use std::process;

use grepzilla::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Bad usage: {err}");
        grepzilla::print_help();
        process::exit(1);
    });

    if let Err(_e) = grepzilla::run(config) {
        process::exit(1);
    }
}
