use std::fmt::Display;

use anyhow;
use colored::*;

struct File {
    pub entry: std::fs::DirEntry
}

impl Display for File {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let file_path = match self.entry.file_name().into_string() {
            Ok(path) => path,
            Err(_) => return write!(f, "Unable to read the file"),
        };

        let metadata = match self.entry.metadata() {
            Ok(metadata) => metadata,
            Err(_) => return write!(f, "Unable to extract the metadata"),
        };

        match metadata {
            _ if metadata.is_file() => return write!(f, "{}", file_path.white()),
            _ if metadata.is_dir() => return write!(f, "{}", file_path.blue()),
            _ if metadata.is_symlink() => return write!(f, "{}", file_path.cyan()),
            _ => return write!(f, "{}", file_path.white()),
        };
    }
}

fn main() -> anyhow::Result<()> {
    let location = std::env::args().nth(1).unwrap_or(String::from("."));
    let dir = std::fs::read_dir(&location)?;

    let mut entries_to_show = dir
        .filter_map(|entry| {
            let entry = entry.ok()?;

            if entry.file_name().into_string().ok()?.starts_with(".") {
                return None;
            }

            return Some(File { entry });
        })
        .collect::<Vec<_>>();

    entries_to_show.sort_by(|a, b| a.entry.path().cmp(&b.entry.path()));

    let entries_to_show = entries_to_show;

    entries_to_show
        .iter()
        .for_each(|file_entry| print!("{} ", file_entry));


    println!("");


    return Ok(());
}
