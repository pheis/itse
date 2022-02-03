// use crate::config::Config;
use clap::Parser;

mod config;

fn main() {
    let config = config::Config::parse();
    println!("Hello, world!");
}
