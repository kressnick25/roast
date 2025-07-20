use assert_cmd::Command;

#[test]
fn version() -> Result<(), String> {
    let mut cmd = Command::cargo_bin("roast").unwrap();

    let version: &str = env!("CARGO_PKG_VERSION");
    let expected = format!("roast {version}\n");

    cmd.arg("--version").assert().success().stdout(expected);

    Ok(())
}

#[test]
fn help_overrides_silent() -> Result<(), String> {
    let mut cmd = Command::cargo_bin("roast").unwrap();

    let version: &str = env!("CARGO_PKG_VERSION");
    let expected = format!("roast {version}\n");

    cmd.arg("--version").arg("--silent").assert().success().stdout(expected);

    Ok(())
}
