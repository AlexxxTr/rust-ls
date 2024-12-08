use anyhow;

fn main() -> anyhow::Result<()> {
    let location = std::env::args().nth(1).unwrap_or(String::from("."));
    let dir = std::fs::read_dir(&location)?;

    let entries_to_show = dir
        .filter_map(|entry| {
            let entry = entry.ok()?;

            if entry.file_name().into_string().ok()?.starts_with(".") {
                return None;
            }

            return Some(entry);
        })
        .collect::<Vec<_>>();

    entries_to_show
        .iter()
        .for_each(|entry| print!("{:?}", entry.file_name()));

    println!("");

    return Ok(());
}
