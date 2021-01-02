use std::path::PathBuf;
use thiserror::Error;

/// All possible errors that could be returned by this library
#[derive(Error, Debug)]
pub enum TemployError {
    #[error("Invalid CLI command")]
    InvalidCLICommand,
    /// Represents an invalid template path
    #[error("Invalid template path specified: {path:?}")]
    InvalidTemplatePath { path: PathBuf },
    /// Represents an invalid deployment path
    #[error("Invalid deployment path specified: {path:?}")]
    InvalidDeploymentPath { path: PathBuf },
    /// Represents an incorrectly-formatted template path
    #[error("Specified template path {path:?} is not of the expected format")]
    InvalidTemplatePathFormat { path: PathBuf },
    #[error("Failed to create directory {name:?} because it already exists")]
    DirAlreadyExists { name: String },
    #[error("Failed to create directory")]
    FailedToCreateDir,
    #[error("Failed to canonicalize directory name")]
    FailedToCanonicalizeDir,
    #[error("Failed to read entry: {source:?}")]
    FailedToReadEntry { source: walkdir::Error },
    #[error("Failed to strip path prefix")]
    FailedToStripPrefix,
    #[error("Invalid GitHub link {link:?}")]
    InvalidGithubLink { link: String },
    #[error("There was a problem cloning from GitHub")]
    GithubCloneError { source: anyhow::Error },
    #[error("Failed to authenticate via GitHub")]
    GithubAuthenticationError,
    /// Represents all other cases of `std::io::Error`
    #[error(transparent)]
    IOError(#[from] std::io::Error),
}
