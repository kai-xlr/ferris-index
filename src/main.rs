use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about = "A simple file indexer built in Rust")]
struct Args {
    /// The directory to scan
    path: PathBuf,

    /// Optional path to save the index (e.g., index.json)
    #[arg(short, long)]
    output: Option<PathBuf>,
}

fn main() {
    let args = Args::parse();

    println!("Scanning: {:?}", args.path);

    if let Some(out) = args.output {
        println!("Index will be saved to: {:?}", out);
    } else {
        println!("No output file specified; results will be printed to stdout.");
    }
}
