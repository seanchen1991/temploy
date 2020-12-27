use anyhow::anyhow;
use clap::{App, Arg, ArgMatches};
use crate::TemployError;

/// Main function that initializes the CLI
fn cli_init() -> Result<(), TemployError> {
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
        ("generate", Some(params)) => ProjectParameters::from_cli(params)?.generate(),
        _ => Err(anyhow!("Invalid command: {}", command)),
    }
}

fn main() {
    if let Err(err) = cli_init() {
        eprintln!("Error: {:?}", err);
        std::process::exit(1);
    }
}
