use core::fmt;

pub enum TailwindcssErrors {
    CurrentDirFailed,
    NotFoundViteConfig,
    ReadViteConfigFailed,
    EditViteConfigFailed,

    FailedToFindTailwindcssImport,
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

            TailwindcssErrors::FailedToFindTailwindcssImport => {
                write!(f, "failed to find tailwindcss import in file")
            }
        }
    }
}
