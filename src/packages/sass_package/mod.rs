use std::path::PathBuf;

use crate::{Framework, packages::npm};
mod setup_sass_files;

pub fn setup(framework: Option<Framework>, current_dir: &PathBuf) {
    npm::npm_command("install,-D,sass");
    setup_sass_files::setup_sass_files(framework, current_dir);
}
