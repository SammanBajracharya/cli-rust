use std::fs;

fn main() {
    let mut paths: Vec<String> = Vec::new();
    for entry in fs::read_dir(".").unwrap() {
        let dir = entry.unwrap();
        let path = dir.path();

        let path_str = path.to_string_lossy();
        let components: Vec<&str> = path_str.split('/').collect();
        if components.len() > 1 {
            paths.push(components[1].to_string());
        }
    }

    paths.sort();

    for path in paths {
        print!("{}  ", path);
    }
    println!();
}
