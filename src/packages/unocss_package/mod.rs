pub mod vite_setup;

use std::path::PathBuf;

use crate::packages;

pub fn setup_unocss(cd: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let _ = packages::utils::npm::npm_command("npm,install,-d,unocss");
    let path = packages::utils::vite_setup::get_vite_conf(&cd)?;
    packages::utils::edit_vite_conf::edit_vite_conf(
        &path,
        "import unocss from 'unocss/vite'",
        "unocss(),",
    )?;

    Ok(())
}
