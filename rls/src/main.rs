use std::fs;
use colored::*;

fn main() {
    // (file type, file name)
    let mut paths: Vec<(String, fs::FileType)> = Vec::new();
    for entry in fs::read_dir(".").unwrap() {
        let dir = entry.unwrap();
        let path = dir.path();

        let path_str = path.to_string_lossy();
        let components: Vec<&str> = path_str.split('/').collect();
        if let Ok(file_type) = dir.file_type() {
            if components.len() > 1 {
                paths.push((components[1].to_string(), file_type));
            }
        }
    }

    // How to sort only 1st part
    paths.sort_by(|a, b| a.0.cmp(&b.0));

    for path in paths {
        let file_name: String = if path.1.is_file() {
            format!("{}  ", path.0.white())
        } else if path.1.is_dir() {
            format!("{}  ", path.0.blue())
        } else {
            format!("{}  ", path.0.red())
        };
        print!("{} ", file_name);
    }
    println!();
}
