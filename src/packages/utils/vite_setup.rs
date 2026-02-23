use crate::packages::error_enums::vite_errors::ViteErrors;
use std::path::PathBuf;

pub fn get_vite_conf(cd: &PathBuf) -> Result<PathBuf, ViteErrors> {
    let vite_conf_file = ["vite.config.ts", "vite.config.js", "vite.config.mjs"];

    for file in vite_conf_file {
        let path = cd.join(file);
        if path.exists() {
            return Ok(path);
        }
    }

    Err(ViteErrors::NotFoundViteConfig)
}
