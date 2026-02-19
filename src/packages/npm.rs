use core::fmt;
use std::process::Command;

pub enum NpmErrors {
    NpmNotFound,
    NpmInstallFailed,
}

impl fmt::Display for NpmErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NpmErrors::NpmNotFound => {
                write!(f, "npm not found on this system")
            }

            NpmErrors::NpmInstallFailed => {
                write!(f, "npm install failed while installing tailwindcss")
            }
        }
    }
}

pub fn npm_command(command: &str) -> Result<(), NpmErrors> {
    let command_parts: Vec<&str> = command.split(",").collect();

    let npm_status = Command::new("npm")
        .args(command_parts)
        .status()
        .map_err(|_| NpmErrors::NpmNotFound)?;

    if !npm_status.success() {
        return Err(NpmErrors::NpmInstallFailed);
    }

    Ok(())
}
