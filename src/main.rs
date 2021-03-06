use anyhow::{anyhow, Result};
use clap::{App, Arg};
use temploy::{DeployParameters, GenerateParameters, TemployError};

/// Main function that initializes the CLI
fn cli_init() -> Result<()> {
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
        .subcommand(
            App::new("deploy")
                .about("Deploy a specified project.")
                .args(&[Arg::with_name("project")
                    .help("Specify project location")
                    .required(true)]),
        )
        .get_matches();

    match matches.subcommand() {
        ("generate", Some(params)) => GenerateParameters::from_cli(params)?.generate(),
        ("deploy", Some(params)) => DeployParameters::from_cli(params)?.deploy(),
        _ => Err(anyhow!(TemployError::InvalidCLICommand)),
    }
}

fn main() {
    if let Err(err) = cli_init() {
        eprintln!("Error: {:?}", err);
        std::process::exit(1);
    }
}
