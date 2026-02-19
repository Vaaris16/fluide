use std::{fs, path::PathBuf};

use crate::{
    Framework,
    packages::{file_errors::FileErrors, make_file::make_file},
};

pub fn setup_sass_files(
    framework: Option<Framework>,
    project_root: &PathBuf,
) -> Result<(), FileErrors> {
    let (relative_path, content) = match framework {
        Some(Framework::Vanilla) => ("src/style.scss", "// Vanilla Sass starter\n"),
        Some(Framework::React) => ("src/index.scss", "// React Sass starter\n"),
        Some(Framework::Preact) => ("src/index.scss", "// Preact Sass starter\n"),
        Some(Framework::Lit) => ("src/index.scss", "// Lit Sass starter\n"),
        Some(Framework::Vue) => ("src/assets/main.scss", "// Vue Sass starter\n"),
        Some(Framework::Svelte) => ("src/app.scss", "// Svelte Sass starter\n"),
        Some(Framework::Solid) => ("src/index.scss", "// Solid Sass starter\n"),
        Some(Framework::Qwik) => ("src/global.scss", "// Qwik Sass starter\n"),
        Some(Framework::Angular) => ("src/styles.scss", "// Angular Sass starter\n"),
        Some(Framework::Ember) => ("app/styles/app.scss", "// Ember Sass starter\n"),
        Some(Framework::Marko) => ("src/style.scss", "// Marko Sass starter\n"),
        None => return Ok(()),
    };

    let file_path = project_root.join(relative_path);

    if let Some(parent) = file_path.parent() {
        fs::create_dir_all(parent).map_err(|_| FileErrors::FailedCreateParentFolders);
    }

    if let Err(err) = make_file(&file_path, content) {
        eprintln!("{}", err)
    }

    Ok(())
}
