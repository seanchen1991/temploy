use anyhow::Result;
use clap::{App, Arg, ArgMatches};

const CONFIG: &str = ".temploy.toml";

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
                        takes_value(true),
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
