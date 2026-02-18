use core::fmt;

pub mod get_vite_conf;
pub mod npm_setup;
pub mod setup_css;

pub fn setup_tailwindcss() -> Result<(), TailwindcssErrors> {
    npm_setup::install_packages_npm()?;
    let path = get_vite_conf::get_vite_conf()?;
    get_vite_conf::add_tailwindcss_vite_conf(&path)?;
    setup_css::setup_index_css()?;
    Ok(())
}

pub enum TailwindcssErrors {
    CurrentDirFailed,
    NotFoundViteConfig,
    ReadViteConfigFailed,
    EditViteConfigFailed,

    NpmNotFound,
    NpmInstallFailed,

    FailedCreateSrc,
    FailedMakingIndexCSS,
    FailedToFindTailwindcssImport,
    FailedWriteToIndexCSS,
}

impl fmt::Display for TailwindcssErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TailwindcssErrors::CurrentDirFailed => {
                write!(f, "failed to get current directory")
            }

            TailwindcssErrors::EditViteConfigFailed => {
                write!(f, "failed to edit vite config file")
            }

            TailwindcssErrors::NotFoundViteConfig => {
                write!(f, "could not fine vite config")
            }

            TailwindcssErrors::ReadViteConfigFailed => {
                write!(f, "could not read vite config")
            }

            TailwindcssErrors::NpmNotFound => {
                write!(f, "npm not found on this system")
            }

            TailwindcssErrors::NpmInstallFailed => {
                write!(f, "npm install failed while installing tailwindcss")
            }

            TailwindcssErrors::FailedCreateSrc => {
                write!(f, "failed to create src")
            }

            TailwindcssErrors::FailedMakingIndexCSS => {
                write!(f, "failed to make index css")
            }

            TailwindcssErrors::FailedToFindTailwindcssImport => {
                write!(f, "failed to find tailwindcss import in index.css")
            }

            TailwindcssErrors::FailedWriteToIndexCSS => {
                write!(f, "failed to write to index css")
            }
        }
    }
}
