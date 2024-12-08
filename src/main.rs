fn main() -> () {
    let location = std::env::args().nth(1).unwrap_or(String::from("."));

    match std::fs::read_dir(&location) {
        Ok(entries) => {
            for entry in entries {
                match entry {
                    Ok(location) => println!("{:?}", location.path()),
                    Err(e) => eprintln!("Error: {}", e),
                }
            }
        },
        Err(e) => eprintln!("Error: {}", e),
    }
}
