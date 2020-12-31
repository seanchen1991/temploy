use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

type Error = Box<dyn std::error::Error>;

#[test]
fn no_template_provided() -> Result<(), Error> {
    let mut cmd = Command::cargo_bin("temploy")?;

    cmd.arg("generate");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("The following required arguments were not provided:"));

    Ok(())
}
