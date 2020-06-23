use std::fs::File;
use std::io::Write;

fn save(data: &String, path: &String) -> std::io::Result<()> {
    let mut file = File::create(path)?;
    file.write_all(data.as_bytes()).expect("Unable to write file");
    Ok(())
}
