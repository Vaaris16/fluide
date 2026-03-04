use file_errors::FileErrors;
use std::path::PathBuf;

use crate::{
    Framework, commands::add_packages::packages::error_enums::file_errors,
    commands::add_packages::utils::npm,
};
mod setup_sass_files;

pub fn setup_sass(framework: Option<Framework>, current_dir: &PathBuf) -> Result<(), FileErrors> {
    let _ = npm::npm_command("install,-D,sass");
    setup_sass_files::setup_sass_files(framework, current_dir)?;

    Ok(())
}
