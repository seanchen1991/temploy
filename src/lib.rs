mod git;

use anyhow::{anyhow, Result};
use clap::ArgMatches;
use heck::KebabCase;
use serde::Deserialize;
use std::{
    env,
    fs::{self, File},
    io::Read,
    path::{Path, PathBuf},
    string::ToString,
};
use thiserror::Error;
use walkdir::{self, WalkDir};

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
    #[error("Invalid CLI command")]
    InvalidCLICommand,
    /// Represents an invalid template path
    #[error("Invalid template path specified: {path:?}")]
    InvalidTemplatePath { path: PathBuf },
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
    /// Represents all other cases of `std::io::Error`
    #[error(transparent)]
    IOError(#[from] std::io::Error),
}

impl ProjectParameters {
    /// Initializes a `ProjectParameters` instance from CLI arguments 
    pub fn from_cli(args: &ArgMatches) -> Result<Self> {
        let template_path = args.value_of("template").unwrap().to_string();

        Ok(ProjectParameters {
            target_dir: args.value_of("target-directory").map(PathBuf::from),
            template_path: PathBuf::from(template_path),
            name: args.value_of("name").map(String::from),
        })
    }
    
    /// Attempts to create the directory where the generated project will live 
    fn create_dir(&self, dir_name: &str) -> Result<PathBuf> {
        let mut dir_path = self.target_dir
            .clone()
            .unwrap_or_else(|| env::current_dir().unwrap_or_else(|_| ".".into()));

        dir_path = dir_path.join(dir_name.to_kebab_case());

        if dir_path.exists() {
            return Err(anyhow!(TemployError::DirAlreadyExists { name: dir_path.to_string_lossy().to_string() }));
        }

        fs::create_dir_all(&dir_path)
            .map_err(|_| anyhow!(TemployError::FailedToCreateDir))?;

        let path = fs::canonicalize(dir_path)
            .map_err(|_| anyhow!(TemployError::FailedToCanonicalizeDir))?;

        Ok(path)
    }

    pub fn generate(&self) -> Result<()> {
        let project_name = match &self.name {
            Some(name) => name.clone(),
            None => {
                // no name provided, get the filename from the template path 
                let path = Path::new(&self.template_path);
                
                if path.is_dir() {
                    path.join(DEFAULT_IDENT).to_string_lossy().to_string()
                } else {
                    return Err(anyhow!(TemployError::InvalidTemplatePath { path: path.to_path_buf() }));
                }
            }
        };

        // attempt to create a new directory where the generated project will live
        let dir_path = self.create_dir(&project_name)?;

        println!("Generating project...");
        
        for entry in WalkDir::new(&self.template_path) {
            let entry = entry.map_err(|err| anyhow!(TemployError::FailedToReadEntry { source: err }))?;
            let entry_path = entry.path().strip_prefix(&self.template_path).map_err(|_| anyhow!(TemployError::FailedToStripPrefix))?;
            let full_path = dir_path.join(entry_path);

            if entry_path == PathBuf::from("") {
                continue;
            }

            if entry.file_type().is_dir() {
                if entry.path().to_str() == Some(".") {
                    continue;
                }

                fs::create_dir(full_path)?;
                continue;
            }

            let filename = entry.path();
            let mut content = String::new();

            {
                let mut file = File::open(filename)?;
                file.read_to_string(&mut content)?;
            }

            fs::write(full_path, content)?;
        }

        println!("Project {} has been successfully generated!", project_name);

        Ok(())
    }
}

