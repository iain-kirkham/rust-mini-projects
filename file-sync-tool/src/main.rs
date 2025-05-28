use std::{fs, io};
use std::path::Path;
use clap::Parser;
use walkdir::WalkDir;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// Path to the source directory
    source: String,

    /// Path to the target directory
    target: String,

    /// Perform a dry run without copying files
    #[arg(short, long)]
    dry_run: bool,
}


fn copy_files(source: &Path, target: &Path, dry_run: bool) {
    for entry in WalkDir::new(source).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();

        if path.is_file() {
            let relative = path.strip_prefix(source).unwrap();
            let target_path = target.join(relative);

            // Check if a target file exists
            if target_path.exists() {
                let source_metadata = fs::metadata(path).unwrap();
                let target_metadata = fs::metadata(&target_path).unwrap();

                if source_metadata.len() == target_metadata.len() {
                    println!("Skipping identical file: {}", relative.display());
                    continue;
                } else {
                    println!("Updating file: {}", relative.display());
                }
            } else {
                println!("Copying new file: {}", relative.display());
            }

            // Ensure parent directory exists
            if let Some(parent) = target_path.parent() {
                fs::create_dir_all(parent).unwrap();
            }

            if dry_run {
                println!("Dry-run would copy: {}", relative.display());
            } else {
                fs::copy(path, target_path).unwrap();
            }
        }
    }
}

fn delete_files(source: &Path, target: &Path, dry_run: bool) {
    let mut entries: Vec<_> = WalkDir::new(target)
        .into_iter()
        .filter_map(|e| e.ok())
        .collect();

    entries.reverse();

    for entry in entries {
        let target_path = entry.path();
        if target_path == target {
            continue;
        }

        let relative = target_path.strip_prefix(target).unwrap();
        let expected_source_path = source.join(relative);

        if !expected_source_path.exists() {
            if dry_run {
                println!("Dry-run: Would delete {}", target_path.display());
            } else {
                if target_path.is_file() {
                    fs::remove_file(target_path).unwrap();
                    println!("Deleted file: {}", target_path.display());
                } else if target_path.is_dir() {
                    fs::remove_dir_all(target_path).unwrap();
                    println!("Deleted directory: {}", target_path.display());
                }
            }
        }
    }
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    let source = Path::new(&args.source);
    let target = Path::new(&args.target);

    if !source.exists() || !target.exists() {
        println!("Source or target directory does not exist.");
        return Ok(());
    }

    copy_files(source, target, args.dry_run);
    delete_files(source, target, args.dry_run);

    println!("Sync complete.");
    Ok(())
}