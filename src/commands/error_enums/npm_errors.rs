use core::fmt;

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
