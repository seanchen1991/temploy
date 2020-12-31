use assert_fs::TempDir;
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

#[test]
fn invalid_template_provided() -> Result<(), Error> {
    let mut cmd = Command::cargo_bin("temploy")?;

    cmd.arg("generate").arg("file/doesnt/exist");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Invalid template path specified:"));

    Ok(())
}

#[test]
fn generates_template_correctly() -> Result<(), Error> {
    let path = "tests/test-data";
    let temp = TempDir::new().unwrap();
    let mut cmd = Command::cargo_bin("temploy")?;

    cmd.arg("generate")
        .arg(path)
        .arg("-d")
        .arg(temp.path());

    cmd.assert().success();
    assert!(dir_diff::is_different(&temp.path(), path).unwrap());

    temp.close().unwrap();

    Ok(())
}
