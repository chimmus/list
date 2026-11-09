pub mod bundle;

use clap::Parser;
use bundle::{env::*, show::*};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    /// all
    #[arg(short = 'a', long = "all")]
    all: bool,
    /// long
    #[arg(short = 'l', long = "long")]
    long: bool,
    /// path
    path: Option<String>,
}

fn main() {
    let args = Cli::parse();
    if args.long {
        todo!("add this");
    }
    let (path, success) = current();
    if success {
        println!("{path}");
    } else {
        println!("error: {path}");
    }
    ls(&path, args.all);

}
