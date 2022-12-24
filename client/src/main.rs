mod args;

use crate::args::Args;
use clap::Parser;

fn main() {
    let args = Args::parse();

    println!("Passed args: {:?}", args);
}
