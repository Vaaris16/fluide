use std::{env, fs};

use crate::packages::{
    file_errors::FileErrors, make_file::make_file, tailwindcss_package::TailwindcssErrors,
};

pub fn setup_index_css() -> Result<(), TailwindcssErrors> {
    let cd = env::current_dir().map_err(|_| TailwindcssErrors::CurrentDirFailed)?;
    let src_cd = cd.join("src");
    let index_css = src_cd.join("index.css");

    let _ = fs::create_dir_all(&src_cd).map_err(|_| FileErrors::FailedCreateParentFolders);

    let tailwindcss_import = "@import \"tailwindcss\";\n";

    if !index_css.exists() {
        if let Err(err) = make_file(&index_css, tailwindcss_import) {
            eprintln!("{}", err);
        }
    }

    let content = fs::read_to_string(&index_css)
        .map_err(|_| TailwindcssErrors::FailedToFindTailwindcssImport)?;

    if !content.contains(tailwindcss_import) {
        let new_contents = format!("{}{}", tailwindcss_import, content);
        let _ = fs::write(index_css, new_contents).map_err(|_| FileErrors::FailedCreateFile);
    }

    Ok(())
}
