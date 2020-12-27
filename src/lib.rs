mod git;

use anyhow::{Context, Result};
use dialoguer::Input;
use heck::KebabCase;
use serde::Deserialize;
use std::{
    env,
    fs::{self, File},
    io::Read,
    path::{Component, PathBuf},
    string::ToString,
};
use thiserror::Error;
use walkdir::WalkDir;

const DEFAULT_IDENT: &'static str = "-clone";

/// Holds all of the parameters for the new project that will be generated
#[derive(Debug, Deserialize)]
pub struct ProjectParameters {
    target_dir: Option<PathBuf>,
    template_path: PathBuf,
    name: Option<String>,
}

/// All possible errors that could be returned by this library
#[derive(Error, Debug)]
pub enum TemployError {
    /// Represents all other cases of `std::io::Error`
    #[error(transparent)]
    IOError(#[from] std::io::Error),
}

impl ProjectParameters {

}

