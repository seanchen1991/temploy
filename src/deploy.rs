use anyhow::{anyhow, Result};
use clap::ArgMatches;
use std::{fs::File, io::Write, path::PathBuf, process::Command};

use crate::TemployError;

const BUILD_OUTPUT: &str = "build-output.txt";

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
        self.build_image()?;

        Ok(())
    }

    fn build_image(&self) -> Result<()> {
        // build the docker image using a Dockerfile located
        // at the root of the project to be deployed
        if !self.to_deploy.exists() {
            return Err(anyhow!(TemployError::InvalidDeploymentPath {
                path: self.to_deploy.clone()
            }));
        }

        let mut build_cmd = Command::new("docker");

        println!("Building Docker image; this might take a few minutes...");

        let output = build_cmd
            .arg("build")
            .arg(&self.to_deploy)
            .output()
            .map_err(|_| anyhow!(TemployError::DockerBuildFailed))?;

        let mut f = File::create(BUILD_OUTPUT)?;

        if output.status.success() {
            f.write_all(&output.stdout).map_err(|_| {
                anyhow!(TemployError::FileWriteFail {
                    filename: String::from(BUILD_OUTPUT)
                })
            })?;
            println!("Successfully build the Docker image! Take a look at the {} file for more information.", BUILD_OUTPUT);
        } else {
            f.write_all(&output.stderr).map_err(|_| {
                anyhow!(TemployError::FileWriteFail {
                    filename: String::from(BUILD_OUTPUT)
                })
            })?;
            println!("Something went wrong when building the Docker image. Take a look at the {} file for more information.", BUILD_OUTPUT);
        }

        Ok(())
    }
}
