use std::env;
use std::collections::HashMap;
use walkdir::WalkDir;
use std::path::{Path, PathBuf};

fn main() {
    // Parse command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <dir1> <dir2>", args[0]);
        return;
    }

    let dir1 = &args[1];
    let dir2 = &args[2];

    // Get file names and their full paths from both directories
    let files_in_dir1 = list_file_names_with_paths(dir1);
    let files_in_dir2 = list_file_names_with_paths(dir2);

    // Compare file names and print full paths of files that are unique to each directory
    let unique_to_dir1 = files_in_dir1
        .iter()
        .filter(|(name, _)| !files_in_dir2.contains_key(*name));
    
    let unique_to_dir2 = files_in_dir2
        .iter()
        .filter(|(name, _)| !files_in_dir1.contains_key(*name));

    // Print files only in dir1 with full paths
    println!("Files only in {}:", dir1);
    for (_, path) in unique_to_dir1 {
        println!("{}", path.display());
    }

    // Print files only in dir2 with full paths
    println!("\nFiles only in {}:", dir2);
    for (_, path) in unique_to_dir2 {
        println!("{}", path.display());
    }
}

// Recursively list all file names with their full paths in a directory
fn list_file_names_with_paths<P: AsRef<Path>>(dir: P) -> HashMap<String, PathBuf> {
    let mut file_map = HashMap::new();
    for entry in WalkDir::new(dir) {
        let entry = entry.unwrap();
        if entry.file_type().is_file() {
            // Extract the file name and store the full path
            if let Some(file_name) = entry.path().file_name() {
                file_map.insert(
                    file_name.to_string_lossy().to_string(),
                    entry.path().to_path_buf(),
                );
            }
        }
    }
    file_map
}