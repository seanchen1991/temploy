mod git;

use anyhow::{anyhow, Result, Context};
use clap::{App, Arg, ArgMatches};
use std::{
    env,
    io::Read,
    fs::{self, File},
    path::{Component, PathBuf},
    string::ToString,
};
use serde::{Serialize, Deserialize};
use dialoguer::Input;
use walkdir::WalkDir;
use heck::KebabCase;

const CONFIG: &str = ".temploy.toml";

/// Holds all of the parameters for the new project that will be generated
#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectParameters {
    target_dir: Option<PathBuf>,
    template_path: PathBuf,
    name: Option<String>,
}

impl ProjectParameters {
    /// Initializes a ProjectParameters instance from CLI arguments 
    pub fn from_cli(matches: &ArgMatches) -> Result<Self> {
        let mut template_path = matches.value_of("template").unwrap().to_string();
        let mut project_parameters: ProjectParameters = {
            // handle template from a GitHub repo
            if template_path.ends_with(".git") {
                let temp_dir = env::temp_dir().join(format!("{:x}", md5::compute(&template_path)));

                if temp_dir.exists() {
                    fs::remove_dir_all(&temp_dir)?;
                }

                fs::create_dir_all(&temp_dir)?;
                git::clone(&template_path, &temp_dir)?;

                template_path = temp_dir.to_string_lossy().to_string();
            }

            let mut config_file = File::open(PathBuf::from(&template_path).join(CONFIG))?;
            let mut config_str = String::new();

            config_file.read_to_string(&mut config_str)?;

            toml::from_str(&config_str)?
        };

        project_parameters.target_dir = matches.value_of("target-directory").map(PathBuf::from);
        project_parameters.template_path = PathBuf::from(template_path);
        project_parameters.name = matches.value_of("name").map(String::from);

        Ok(project_parameters)
    }

    fn create_dir(&self, name: &str) -> Result<PathBuf> {
        let mut dir_path = self.target_dir
            .clone()
            .unwrap_or_else(|| env::current_dir().unwrap_or_else(|_| ".".into()));

        dir_path = dir_path.join(name.to_kebab_case());

        if dir_path.exists() {
            return Err(anyhow!("Failed to create {} since it already exists", dir_path.to_string_lossy()));
        }

        fs::create_dir_all(&dir_path).with_context(|| "Cannot create directory")?;

        let path = fs::canonicalize(dir_path).with_context(|| "Cannot canonicalize path")?;

        Ok(path)
    }

    pub fn generate(&self) -> Result<()> {
        let project_name = match &self.name {
            Some(name) => name.clone(),
            None => Input::new()
                .with_prompt("Specify a name for your generated project: ")
                .interact()?,
        };

        let dir_path = self.create_dir(&project_name)?;

        let entries = WalkDir::new(&self.template_path)
            .into_iter()
            .filter_entry(|e| {
                if e.path()
                    .components()
                    .any(|c| c == Component::Normal(".git".as_ref())) 
                {
                    return false;        
                }

                !(e.file_name() == CONFIG)
            });
        
        println!("Generating project...");

        for e in entries {
            let entry = e.map_err(|err| anyhow!("Cannot read entry: {}", err))?;
            let entry_path = entry.path().strip_prefix(&self.template_path)?;
            let full_path = dir_path.join(entry_path);

            if entry_path == PathBuf::from("") {
                continue;
            }

            if entry.file_type().is_dir() {
                if entry.path().to_str() == Some(".") {
                    continue;
                }

                fs::create_dir(full_path)
                    .map_err(|err| anyhow!("Cannot create directory: {}", err))?;
                continue;
            }

            let filename = entry.path();
            let mut content = String::new();

            {
                let mut file = File::open(filename)
                    .map_err(|err| anyhow!("Cannot open file: {}", err))?;
                file.read_to_string(&mut content)
                    .map_err(|err| anyhow!("Cannot read file: {}", err))?;
            }

            fs::write(full_path, content)
                .map_err(|err| anyhow!("Failed to write to file: {}", err))?;
        }

        println!("Project {} has been successfully generated", project_name);

        Ok(())
    }
}

/// Main function that initializes the CLI
pub fn cli_init() -> Result<()> {
    let matches = App::new("temploy")
        .subcommand(
            App::new("generate")
                .about("Generate a new project from a specified template.")
                .args(&[
                    Arg::with_name("template")
                        .help("Specify template location")
                        .required(true),
                    Arg::with_name("name")
                        .short("n")
                        .long("name")
                        .help("Specify the name of your generated project.")
                        .takes_value(true),
                    Arg::with_name("target-directory")
                        .short("d")
                        .long("target-directory")
                        .help("Specify the target directory.")
                        .takes_value(true),
                ]),
        )
        .get_matches();

    match matches.subcommand() {
        ("generate", Some(cmd)) => ProjectParameters::from_cli(cmd)?.generate(),
        _ => Err(anyhow!("Invalid command")),
    }
}
