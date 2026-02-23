use clap::Parser;
use serde_derive::Serialize;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

#[derive(Parser, Debug)]
#[command(author, version, about = "A simple file indexer built in Rust")]
struct Args {
    path: PathBuf,
    #[arg(short, long)]
    output: Option<PathBuf>,
}

#[derive(Debug, Serialize)]
struct FileInfo {
    path: PathBuf,
    size: u64,
    modified: SystemTime,
}

fn visit_dirs(path: &Path) -> std::io::Result<Vec<FileInfo>> {
    let mut files = Vec::new();

    if !path.is_dir() || path.is_symlink() {
        return Ok(files);
    }

    let entries = match fs::read_dir(path) {
        Ok(e) => e,
        Err(e) => {
            eprintln!("Skipping {:?}: {}", path, e);
            return Ok(files);
        }
    };

    for entry in entries {
        let entry = entry?;
        let file_path = entry.path();

        if file_path.is_dir() {
            files.extend(visit_dirs(&file_path)?);
        } else {
            let metadata = entry.metadata()?;
            files.push(FileInfo {
                path: file_path,
                size: metadata.len(),
                modified: metadata.modified()?,
            });
        }
    }

    Ok(files)
}

fn main() {
    let args = Args::parse();

    match visit_dirs(&args.path) {
        Ok(files) => {
            println!("Found {} files", files.len());
        }
        Err(e) => {
            eprintln!("Critical error during scan: {}", e);
            std::process::exit(1);
        }
    }
}
