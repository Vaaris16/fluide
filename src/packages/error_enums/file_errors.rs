use core::fmt;

#[derive(Debug)]
pub enum FileErrors {
    FailedCreateFile,
    FailedCreateParentFolders,
    FailedToGetCurrentDir,
}

impl fmt::Display for FileErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FileErrors::FailedCreateFile => {
                write!(f, "failed to make file")
            }

            FileErrors::FailedCreateParentFolders => {
                write!(f, "failed to create parent folders")
            }

            FileErrors::FailedToGetCurrentDir => {
                write!(f, "failed to get current director")
            }
        }
    }
}
