use std::process::Command;

use crate::packages::tailwindcss_package::TailwindcssErrors;

pub fn install_packages_npm() -> Result<(), TailwindcssErrors> {
    let npm_status = Command::new("npm")
        .args(["install", "tailwindcss", "@tailwindcss/vite"])
        .status()
        .map_err(|_| TailwindcssErrors::NpmNotFound)?;

    if !npm_status.success() {
        return Err(TailwindcssErrors::NpmInstallFailed);
    }

    Ok(())
}
