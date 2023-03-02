use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertions
use std::process::Command; // Run programs

#[test]
fn empty_runs_show_help() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Usage:"));
    Ok(())
}

#[test]
fn build_subcommand_required_directory() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;

    cmd.arg("build")
        .assert()
        .failure()
        .stderr(predicate::str::contains("--dir <ROOT_DIRECTORY>"));
    Ok(())
}
