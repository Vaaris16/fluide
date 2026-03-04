use crate::commands::error_enums::npm_errors::NpmErrors;
use std::process::Command;

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
