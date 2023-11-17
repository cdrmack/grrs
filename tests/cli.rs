use assert_cmd::prelude::*; // add methods on commands
use predicates::prelude::*; // used for writing assertions
use std::process::Command; // run programs

#[test]
fn file_does_not_exist() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd =  Command::cargo_bin("grrs")?;

    cmd.arg("samplepattern").arg("invalid/missingfile");
    cmd.assert().failure().stderr(predicate::str::contains("could not open file"));

    Ok(())
}
