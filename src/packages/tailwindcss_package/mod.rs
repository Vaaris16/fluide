pub mod get_vite_conf;
pub mod setup_css;

use tailwindcss::TailwindcssErrors;

use crate::packages;
mod tailwindcss;

pub fn setup_tailwindcss() -> Result<(), TailwindcssErrors> {
    let _ = packages::npm::npm_command("install,tailwindcss,@tailwindcss/vite");
    let path = get_vite_conf::get_vite_conf()?;
    get_vite_conf::add_tailwindcss_vite_conf(&path)?;
    setup_css::setup_index_css()?;
    Ok(())
}
