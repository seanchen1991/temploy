use anyhow::Result;
use clap::{App, Arg, ArgMatches};
use std::{
    collections::HashMap,
    env,
    fs::{self, File},
    path::PathBuf,
    string::ToString,
};
use toml::Value;

const CONFIG: &str = ".temploy.toml";

/// Holds all of the parameters for the new project that will be generated
pub struct ProjectParameters {
    target_dir: PathBuf,
    template_path: Option<PathBuf>,
    name: Option<String>,
}

impl ProjectParameters {
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
                git::clone(&template_path, &temp_dir, matches.is_present("passphrase"))?;

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
                    Arg::with_name("password")
                        .short("p")
                        .long("password")
                        .help("Specify if your SSH key is protected by a password")
                        .takes_value(false),
                ]),
        )
        .get_matches();

    match matches.subcommand() {
        ("generate", Some(cmd)) => ProjectParameters::from_cli(cmd)?.generate(),
        _ => Err(anyhow!("Invalid command")),
    }
}
