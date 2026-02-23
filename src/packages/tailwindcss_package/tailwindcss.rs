use core::fmt;

#[derive(Debug)]
pub enum TailwindcssErrors {
    CurrentDirFailed,
    FailedToFindTailwindcssImport,
}

impl fmt::Display for TailwindcssErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TailwindcssErrors::CurrentDirFailed => {
                write!(f, "failed to get current directory")
            }

            TailwindcssErrors::FailedToFindTailwindcssImport => {
                write!(f, "failed to find tailwindcss import in file")
            }
        }
    }
}

impl std::error::Error for TailwindcssErrors {}
