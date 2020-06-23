use std::fs::{File, write};
use std::path::Path;

fn save(data: &String, path: Path) -> std::io::Result<()> {
    let mut file = File::create(path)?;
    write(path, data).expect("Unable to write file");
    Ok(())
}
