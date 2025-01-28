use std::{env::args, fs, os::unix::fs::{FileTypeExt, MetadataExt}, process};
use chrono::{Utc, TimeZone};
use std::time::{UNIX_EPOCH, SystemTime};
use colored::*;
use users::{get_user_by_uid, get_group_by_gid};

enum DisplayMode {
    Normal,
    Long,           // -l
    All,            // -a
    Recursive,      // -R
    HumanReadble,   // -lh
}

fn format_permissions(mode: u32) -> String {
    let mut permissions = String::new();
    permissions.push(if mode & 0o400 != 0 { 'r' } else { '-' });
    permissions.push(if mode & 0o200 != 0 { 'w' } else { '-' });
    permissions.push(if mode & 0o100 != 0 { 'x' } else { '-' });
    permissions.push(if mode & 0o040 != 0 { 'r' } else { '-' });
    permissions.push(if mode & 0o020 != 0 { 'w' } else { '-' });
    permissions.push(if mode & 0o010 != 0 { 'x' } else { '-' });
    permissions.push(if mode & 0o004 != 0 { 'r' } else { '-' });
    permissions.push(if mode & 0o002 != 0 { 'w' } else { '-' });
    permissions.push(if mode & 0o001 != 0 { 'x' } else { '-' });
    permissions
}

fn format_time(modified: SystemTime) -> String {
    let datetime = modified
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    let secs = datetime.as_secs();
    let naive = Utc.timestamp_opt(secs as i64, 0)
        .single()
        .expect("Invalid timestamp");

    naive.format("%b %d %H:%M").to_string()
}

fn main() {
    let args: Vec<String> = args().collect();

    if args.len() > 2 {
        println!("ls: cannot access '{}'", args[2]);
        process::exit(1);
    }

    let mode: DisplayMode = if args[1] == "-l" {
        DisplayMode::Long
    } else if args[1] == "-a" {
        DisplayMode::All
    } else if args[1] == "-R" {
        DisplayMode::Recursive
    } else if args[1] == "-lh" {
        DisplayMode::HumanReadble
    } else {
        DisplayMode::Normal
    };

    // (file type, file name)
    let mut paths: Vec<(String, fs::Metadata)> = Vec::new();
    for entry in fs::read_dir(".").unwrap() {
        let dir = entry.unwrap();
        let path = dir.path();
        let metadata = dir.metadata().unwrap();

        if let DisplayMode::All = mode {
            paths.push((path.to_string_lossy().to_string(), metadata));
        } else if !path.file_name().unwrap().to_string_lossy().starts_with('.') {
            paths.push((path.to_string_lossy().to_string(), metadata));
        }
    }

    paths.sort_by(|a, b| a.0.cmp(&b.0));

    let max_size_width = paths.iter().map(|(_, meta)| meta.size().to_string().len()).max().unwrap_or(0);
    let max_name_width = paths.iter().map(|(name, _)| name.len()).max().unwrap_or(0);

    for (path, metadata) in paths {
        let file_name: String = if metadata.is_file() {
            format!("{}  ", path.white())
        } else if metadata.is_dir() {
            format!("{}  ", path.blue())
        } else if metadata.is_symlink() {
            format!("{}  ", path.cyan())
        } else if metadata.file_type().is_socket() {
            format!("{}  ", path.magenta())
        } else if metadata.file_type().is_fifo() {
            format!("{}  ", path.yellow())
        } else if metadata.file_type().is_block_device() || metadata.file_type().is_char_device() {
            format!("{}  ", path.yellow().on_black())
        } else {
            format!("{}  ", path.red())
        };

        match mode {
            DisplayMode::Normal => print!("{} ", file_name),
            DisplayMode::Long => {

                let file_type = if metadata.file_type().is_dir() {
                    "d"
                } else if metadata.file_type().is_symlink() {
                    "l"
                } else if metadata.file_type().is_file() {
                    "-"
                } else {
                    "?"
                };
                let permissions = format_permissions(metadata.mode());
                let hard_links = metadata.nlink();
                let owner_id = metadata.uid();
                let group_id = metadata.gid();
                let size = metadata.size();
                let modified_time = format_time(metadata.modified().unwrap());

                let user = get_user_by_uid(owner_id).map_or_else(|| owner_id.to_string(), |u| u.name().to_string_lossy().into_owned());
                let group = get_group_by_gid(group_id).map_or_else(|| group_id.to_string(), |g| g.name().to_string_lossy().into_owned());

                println!(
                    "{}{} {:2} {:5} {:5} {:>width$} {} {:<name_width$}",
                    file_type,
                    permissions,
                    hard_links,
                    user,
                    group,
                    size,
                    modified_time,
                    file_name,
                    width = max_size_width,
                    name_width = max_name_width,
                );
            },
            _ => {}
        }
    }
    println!();
}
