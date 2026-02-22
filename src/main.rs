use clap::Parser;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Parser, Debug)]
#[command(author, version, about = "A simple file indexer built in Rust")]
struct Args {
    path: PathBuf,
    #[arg(short, long)]
    output: Option<PathBuf>,
}

fn visit_dirs(path: &Path) -> std::io::Result<()> {
    // Check if it's a directory and not a symlink to avoid infinite loops
    if path.is_dir() && !path.is_symlink() {
        // Handle potential permission errors by matching on the result
        let entries = match fs::read_dir(path) {
            Ok(entries) => entries,
            Err(e) => {
                eprintln!("Skipping {:?}: {}", path, e);
                return Ok(()); // Skip this dir but keep the rest of the scan going
            }
        };

        for entry in entries {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path)?;
            } else {
                println!("{}", path.display());
            }
        }
    }
    Ok(())
}

fn main() {
    let args = Args::parse();

    // Call our function and handle the final result
    if let Err(e) = visit_dirs(&args.path) {
        eprintln!("Critical error during scan: {}", e);
        std::process::exit(1);
    }
}
