use crate::{errors::TemployError, git};
use anyhow::{anyhow, Context, Result};
use clap::ArgMatches;
use heck::KebabCase;
use serde::Deserialize;
use std::{
    env,
    fs::{self, File},
    io::Read,
    path::{Component, Path, PathBuf},
    string::ToString,
};
use walkdir::{self, WalkDir};

const DEFAULT_IDENT: &str = "-clone";

/// Holds all of the parameters for the new project that will be generated
#[derive(Debug, Deserialize)]
pub struct GenerateParameters {
    target_dir: Option<PathBuf>,
    template_path: PathBuf,
    name: Option<String>,
    gh_repo_name: Option<String>,
}

impl GenerateParameters {
    /// Initializes a `GenerateParameters` instance from CLI arguments
    pub fn from_cli(args: &ArgMatches) -> Result<Self> {
        let mut gh_repo_name = None;
        let mut template_path = args.value_of("template").unwrap().to_string();

        // a github repo was specified
        // clone down the contents of the repo into a temp directory
        // TODO: Change this flow so that it only makes one directory-creation pass
        if template_path.ends_with(".git") {
            let temp_dir = env::temp_dir().join(format!("{:x}", md5::compute(&template_path)));

            if temp_dir.exists() {
                fs::remove_dir_all(&temp_dir).context("Failed to remove temporary directory")?;
            }

            fs::create_dir_all(&temp_dir).context("Failed to create temporary directory")?;

            git::clone(&template_path, &temp_dir)
                .map_err(|err| anyhow!(TemployError::GithubCloneError { source: err }))?;

            gh_repo_name = Some(template_path.to_string());
            template_path = temp_dir.to_string_lossy().to_string();
        }

        Ok(GenerateParameters {
            target_dir: args.value_of("target-directory").map(PathBuf::from),
            template_path: PathBuf::from(template_path),
            name: args.value_of("name").map(String::from),
            gh_repo_name,
        })
    }

    /// Attempts to create the directory where the generated project will live
    fn create_dir(&self, dir_name: &str) -> Result<PathBuf> {
        let mut dir_path = self
            .target_dir
            .clone()
            .unwrap_or_else(|| env::current_dir().unwrap_or_else(|_| ".".into()));

        dir_path = dir_path.join(dir_name.to_kebab_case());

        if dir_path.exists() {
            return Err(anyhow!(TemployError::DirAlreadyExists {
                name: dir_path.to_string_lossy().to_string()
            }));
        }

        fs::create_dir_all(&dir_path).map_err(|_| anyhow!(TemployError::FailedToCreateDir))?;

        let path = fs::canonicalize(dir_path)
            .map_err(|_| anyhow!(TemployError::FailedToCanonicalizeDir))?;

        Ok(path)
    }
    
    /// Generates a new project from the specified template directory
    pub fn generate(&self) -> Result<()> {
        let project_name: String = match &self.name {
            Some(name) => name.clone(),
            None => {
                if self.gh_repo_name.is_some() {
                    // get the name of the github repo
                    let template_path_str = self.gh_repo_name.as_ref().unwrap().clone();
                    let mut repo_name = template_path_str
                        .split('/')
                        .last()
                        .ok_or_else(|| {
                            anyhow!(TemployError::InvalidGithubLink {
                                link: template_path_str.clone()
                            })
                        })
                        .unwrap();

                    repo_name = repo_name
                        .split('.')
                        .next()
                        .ok_or_else(|| {
                            anyhow!(TemployError::InvalidGithubLink {
                                link: template_path_str.clone()
                            })
                        })
                        .unwrap();

                    format!("{}{}", repo_name, DEFAULT_IDENT)
                } else {
                    // no name provided, get the filename from the template path
                    let path = Path::new(&self.template_path);

                    if path.is_dir() {
                        path.join(DEFAULT_IDENT).to_string_lossy().to_string()
                    } else {
                        return Err(anyhow!(TemployError::InvalidTemplatePath {
                            path: path.to_path_buf()
                        }));
                    }
                }
            }
        };

        // attempt to create a new directory where the generated project will live
        let dir_path = self.create_dir(&project_name)?;

        // filter out directory entries we don't want to copy
        let entries = WalkDir::new(&self.template_path)
            .into_iter()
            .filter_entry(|e| {
                !e.path()
                    .components()
                    .any(|c| c == Component::Normal(".git".as_ref()))
            });

        println!("Generating project...");

        for entry in entries {
            let entry =
                entry.map_err(|err| anyhow!(TemployError::FailedToReadEntry { source: err }))?;
            let entry_path = entry
                .path()
                .strip_prefix(&self.template_path)
                .map_err(|_| anyhow!(TemployError::FailedToStripPrefix))?;
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
                let mut file =
                    File::open(filename).context(format!("Unable to open {:#?}", filename))?;
                file.read_to_string(&mut content)
                    .context(format!("Unable to read {:#?}", filename))?;
            }

            fs::write(&full_path, content)
                .context(format!("Unable to write to {:#?}", full_path))?;
        }

        println!("Project {} has been successfully generated!", project_name);

        Ok(())
    }
}
