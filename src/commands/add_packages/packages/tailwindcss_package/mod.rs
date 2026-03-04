pub mod setup_css;

use std::path::PathBuf;

use crate::commands::utils;
mod tailwindcss;

pub fn setup_tailwindcss(cd: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let _ = utils::npm::npm_command("install,tailwindcss,@tailwindcss/vite");
    let path = utils::vite_setup::get_vite_conf(&cd)?;
    utils::edit_vite_conf::edit_vite_conf(
        &path,
        "import tailwindcss from '@tailwindcss/vite'",
        "tailwindcss()",
    )?;
    setup_css::setup_index_css()?;
    Ok(())
}
