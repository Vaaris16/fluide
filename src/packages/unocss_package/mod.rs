use std::path::PathBuf;

use crate::packages;

pub fn setup_unocss(cd: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let _ = packages::utils::npm::npm_command("install,-D,unocss");
    let path = packages::utils::vite_setup::get_vite_conf(&cd)?;
    packages::utils::edit_vite_conf::edit_vite_conf(
        &path,
        "import UnoCSS from 'unocss/vite'",
        "UnoCSS(),",
    )?;

    packages::utils::make_file::make_file(
        &cd.join("uno.config.ts"),
        r#"import { defineConfig } from 'unocss'

export default defineConfig({})
"#,
    )?;

    let main_file = packages::utils::find_main::find_main(&cd)?;
    packages::utils::edit_file::edit_file(&main_file, "import 'virtual:uno.css'")?;

    Ok(())
}
