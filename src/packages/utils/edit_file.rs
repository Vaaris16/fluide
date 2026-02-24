use std::{fs, path::PathBuf};

pub fn edit_file(path: &PathBuf, import_line: &str) -> std::io::Result<()> {
    let mut content = fs::read_to_string(path)?;

    if content.contains(import_line) {
        return Ok(());
    }

    content = format!("{}\n{}", import_line, content);

    fs::write(path, content)?;

    Ok(())
}
