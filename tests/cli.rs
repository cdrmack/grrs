use assert_cmd::prelude::*; // add methods on commands
use predicates::prelude::*; // used for writing assertions
use std::process::Command; // run programs
use assert_fs::prelude::*; // creates and deletes files at runtime

#[test]
fn file_does_not_exist() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd =  Command::cargo_bin("grrs")?;
    cmd.arg("samplepattern").arg("missingfile");
    cmd.assert().failure().stderr(predicate::str::contains("could not open file"));

    Ok(())
}

#[test]
fn find_content_in_file() -> Result<(), Box<dyn std::error::Error>> {
    let file = assert_fs::NamedTempFile::new("sample.txt")?;
    file.write_str("A test\nActual content\nMore content\nAnother test")?;

    let mut cmd = Command::cargo_bin("grrs")?;
    cmd.arg("test").arg(file.path());
    cmd.assert().success().stdout(predicate::str::contains("A test\nAnother test"));

    Ok(())
}

#[test]
fn pattern_is_empty() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("grrs")?;
    cmd.arg("").arg("missingfile");
    cmd.assert().failure().stderr(predicate::str::contains("pattern cannot be empty"));

    Ok(())
}
