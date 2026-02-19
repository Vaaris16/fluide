use std::{fs, path::PathBuf};

use crate::packages::file_errors::FileErrors;

pub fn make_file(path: &PathBuf, content: &str) -> Result<(), FileErrors> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|_| FileErrors::FailedCreateParentFolders)?;
    }

    fs::write(path, content).map_err(|_| FileErrors::FailedCreateFile)?;

    print!("file create");

    Ok(())
}
