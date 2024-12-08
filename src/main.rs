use anyhow;

fn main() -> anyhow::Result<()> {
    let location = std::env::args().nth(1).unwrap_or(String::from("."));
    let dir = std::fs::read_dir(&location)?;

    let mut entries_to_show = dir
        .filter_map(|entry| {
            let entry = entry.ok()?;

            if entry.file_name().into_string().ok()?.starts_with(".") {
                return None;
            }

            return Some(entry);
        })
        .collect::<Vec<_>>();

    entries_to_show.sort_by(|a, b| a.path().cmp(&b.path()));

    let entries_to_show = entries_to_show;

    entries_to_show
        .iter()
        .filter_map(|entry| entry.file_name().into_string().ok())
        .for_each(|name| print!("{} ", name));


    println!("");


    return Ok(());
}
