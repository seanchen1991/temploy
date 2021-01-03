use std::path::PathBuf;
use thiserror::Error;

/// All possible errors that could be returned by this library
#[derive(Error, Debug)]
pub enum TemployError {
    /// Represents an invalid command that was passed to the CLI
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
    /// Represents a failure to create a directory because it already exists
    #[error("Failed to create directory {name:?} because it already exists")]
    DirAlreadyExists { name: String },
    /// Represents a failure to create a directory
    #[error("Failed to create directory")]
    FailedToCreateDir,
    /// Represents a failure when canonicalizing a directory name
    #[error("Failed to canonicalize directory name")]
    FailedToCanonicalizeDir,
    /// Represents a failure when attempting to read a directory
    #[error("Failed to read entry: {source:?}")]
    FailedToReadEntry { source: walkdir::Error },
    /// Represents a failure when attempting to strip the prefix from a path
    #[error("Failed to strip path prefix")]
    FailedToStripPrefix,
    /// Represents an invalid github link
    #[error("Invalid GitHub link {link:?}")]
    InvalidGithubLink { link: String },
    /// Represents a problem when attempting to clone a repo from github
    #[error("There was a problem cloning from GitHub")]
    GithubCloneError { source: anyhow::Error },
    /// Represents a failure when attempting to authenticate via github
    #[error("Failed to authenticate via GitHub")]
    GithubAuthenticationError,
    /// Represents a failure when attempting to write to the specified file
    #[error("Failed to write to a file: {filename:?}")]
    FileWriteFail { filename: String },
    /// Represents a failure when attempting to build a Docker image
    #[error("Something went wrong when building the Docker image")]
    DockerBuildFailed,
    /// Represents a failure when attempting to deploy to Digital Ocean
    #[error("Something went wrong when attempting to deploy to Digital Ocean")]
    DODeployFailed,
    /// Represents all other cases of `std::io::Error`
    #[error(transparent)]
    IOError(#[from] std::io::Error),
}
