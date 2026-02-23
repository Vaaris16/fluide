use core::fmt;

#[derive(Debug)]
pub enum ViteErrors {
    NotFoundViteConfig,
    ReadViteConfigFailed,
    EditViteConfigFailed,
}

impl fmt::Display for ViteErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ViteErrors::EditViteConfigFailed => {
                write!(f, "failed to edit vite config file")
            }

            ViteErrors::NotFoundViteConfig => {
                write!(f, "could not fine vite config")
            }

            ViteErrors::ReadViteConfigFailed => {
                write!(f, "could not read vite config")
            }
        }
    }
}

impl std::error::Error for ViteErrors {}
