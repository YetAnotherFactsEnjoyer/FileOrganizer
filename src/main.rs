use clap::Parser;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use anyhow::{Context, Result};


#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    #[arg(short, long)]
    src: PathBuf,

    #[arg(short, long)]
    dst: PathBuf,

    #[arg(short, long, default_value = "by-type")]
    mode: String,

    #[arg(long)]
    dry_run: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

    if !args.src.exists() || !args.src.is_dir() {
        anyhow::bail!("Source path is not a valid Directory");
    }

    fs::create_dir_all(&args.dst)
        .with_context(|| format!("Failed to Create Destionation"));

    for entry in WalkDir::new(&args.src).into_iter().filter_map(|e| e.ok()){
        let path = entry.path(); 

        if path.is_dir() {
            continue;
        }

        let ext = path
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("unknow")
            .to_lowercase();

        let target_dir = args.dst.join(&ext);
        let file_name = path.file_name().unwrap();
        let target_path = target_dir.join(file_name);

        fs::create_dir_all(&target_dir)
            .with_context(|| format!("Failed to create dir {:?}", target_dir))?;

        if args.dry_run {
            println!("[Dry Run] Move {:?} -> {:?}", path, target_path);
        } else {
            fs::rename(path, &target_path)
                .with_context(|| format!("Failed to move {:?} -> {:?}", path, target_path))?;
            println!("Moved {:?} -> {:?}", path, target_path);
        }
    }
    Ok(())
}
