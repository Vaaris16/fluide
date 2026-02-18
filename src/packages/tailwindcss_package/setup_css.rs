use std::{env, fs};

use crate::packages::tailwindcss_package::TailwindcssErrors;

pub fn setup_index_css() -> Result<(), TailwindcssErrors> {
    let cd = env::current_dir().map_err(|_| TailwindcssErrors::CurrentDirFailed)?;
    let src_cd = cd.join("src");
    let index_css = src_cd.join("index.css");

    fs::create_dir_all(&src_cd).map_err(|_| TailwindcssErrors::FailedCreateSrc)?;

    if !index_css.exists() {
        fs::write(&index_css, "@import \"tailwindcss\";\n")
            .map_err(|_| TailwindcssErrors::FailedWriteToIndexCSS)?;
        return Ok(());
    }

    let content = fs::read_to_string(&index_css)
        .map_err(|_| TailwindcssErrors::FailedToFindTailwindcssImport)?;

    if !content.contains("@import \"tailwindcss\";\n") {
        let new_contents = format!("@import \"tailwindcss\";\n{}", content);
        let _ =
            fs::write(index_css, new_contents).map_err(|_| TailwindcssErrors::FailedMakingIndexCSS);
    }

    Ok(())
}
