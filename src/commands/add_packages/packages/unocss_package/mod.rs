use std::path::PathBuf;

use crate::commands::add_packages::utils;

pub fn setup_unocss(cd: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let _ = utils::npm::npm_command("install,-D,unocss");
    let path = utils::vite_setup::get_vite_conf(&cd)?;
    utils::edit_vite_conf::edit_vite_conf(&path, "import UnoCSS from 'unocss/vite'", "UnoCSS(),")?;

    utils::make_file::make_file(
        &cd.join("uno.config.ts"),
        r#"import { defineConfig } from 'unocss'

export default defineConfig({})
"#,
    )?;

    let main_file = utils::find_main::find_main(&cd)?;
    utils::edit_file::edit_file(&main_file, "import 'virtual:uno.css'")?;

    Ok(())
}
