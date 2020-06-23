#![allow(dead_code)]

use std::fs::File;
use std::io::Write;

fn save(data: &str, path: &str) -> std::io::Result<()> {
    let mut file = File::create(path)?;
    file.write_all(data.as_bytes())
        .expect("Unable to write file");
    Ok(())
}
