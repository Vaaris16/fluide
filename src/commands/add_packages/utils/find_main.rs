use std::path::PathBuf;

use crate::commands::add_packages::packages::error_enums::vite_errors::ViteErrors;

pub fn find_main(cd: &PathBuf) -> Result<PathBuf, ViteErrors> {
    let candidates = ["src/main.ts", "src/main.tsx", "src/main.js", "src/main.jsx"];

    for file in candidates {
        let path = cd.join(file);
        if path.exists() {
            return Ok(path);
        }
    }

    Err(ViteErrors::NotFoundMainFile)
}
