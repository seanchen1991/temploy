use anyhow::{anyhow, Result};
use clap::ArgMatches;
use std::{
    io::{self, Write},
    path::PathBuf,
    process::Command,
};

use crate::TemployError;

pub struct DeployParameters {
    /// Path to the project to be deployed
    to_deploy: PathBuf, 
}

impl DeployParameters {
    pub fn from_cli(args: &ArgMatches) -> Result<Self> {
        let to_deploy = args.value_of("project").unwrap().to_string();

        Ok(DeployParameters {
            to_deploy: PathBuf::from(to_deploy),
        })
    }

    pub fn deploy(&self) -> Result<()> {
        // build the docker image using a Dockerfile located 
        // at the root of the project to be deployed
        if !self.to_deploy.exists() {
            return Err(anyhow!(TemployError::InvalidDeploymentPath { path: self.to_deploy.clone() }));
        }

        let mut build_cmd = Command::new("docker");

        println!("Building Docker image; this might take a few minutes...");

        let output = build_cmd.arg("build")
            .arg(&self.to_deploy)
            .output()
            .expect("Failed to run `docker build`");
        
        io::stdout().write_all(&output.stdout).unwrap();
        io::stderr().write_all(&output.stderr).unwrap();

        Ok(())
    }
}
