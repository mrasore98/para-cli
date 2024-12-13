use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;
use assert_fs;
use anyhow::Result;
use std::env::set_current_dir;

#[test]
fn test_init() -> Result<()> {
    let temp_dir = assert_fs::TempDir::new()?;

    set_current_dir(temp_dir.path())?;
    let mut cmd = Command::cargo_bin("para")?;
    cmd.arg("--test").arg("init").arg("--force");
    cmd.assert()
        .success();
    
    Ok(())
}