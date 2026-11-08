pub mod bundle;

use clap::Parser;
use bundle::env::*;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    /// all
    #[arg(short = 'a', long = "all")]
    all: bool,
    /// long
    #[arg(short = 'l', long = "long")]
    long: bool,
}

fn main() {
    let args = Cli::parse();
    let (path, success) = current();
    if success {
        println!("{path}");
    } else {
        println!("error: {path}");
    }
    if args.long && args.all {
        todo!("add this");
    }
}
