mod backend;
mod cli_parsing;
mod errors;
mod models;
mod modes;

use clap::Parser; // Needs to be in scope for the derive macro (::parse()) to work here.

// const MOCK_LOG_PATH: &str = "mockdb/logs.txt";

fn main() {
    backend::setup();

    let args = cli_parsing::Args::parse();

    dbg!(&args);

    cli_parsing::dispatch(&args);
}
