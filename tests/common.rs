use std::fs::File;
use std::io::{self, Write};
use std::path::Path;

pub fn write_to_file_for_test(file_path: &str, content: &str) -> io::Result<()> {
    let path = Path::new(file_path);
    let mut file = File::create(path)?;

    file.write_all(content.as_bytes())?;
    Ok(())
}