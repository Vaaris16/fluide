use crate::packages::error_enums::vite_errors::ViteErrors;
use std::{fs, path::PathBuf};

pub fn edit_vite_conf(path: &PathBuf, import_line: &str, plugin: &str) -> Result<(), ViteErrors> {
    let mut content = fs::read_to_string(path).map_err(|_| ViteErrors::ReadViteConfigFailed)?;

    if !content.contains(import_line) {
        content = format!("{}\n{}", import_line, content)
    }

    if content.contains("plugins: [") && !content.contains(plugin) {
        content = content.replace("plugins: [", &format!("plugins: [{}, ", plugin));
    }

    if content.contains("plugins: [") {
        content = content.replace(
            "defineConfig({",
            &format!("defineConfig({{\n  plugins: [{}],", plugin),
        );
    }

    let _ = fs::write(path, content).map_err(|_| ViteErrors::EditViteConfigFailed);

    Ok(())
}
