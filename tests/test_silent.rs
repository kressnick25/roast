use assert_cmd::Command;
use assertables::{assert_contains, assert_is_empty};

#[cfg(windows)]
const BINARY_NAME: &str = "roast.exe";
#[cfg(not(windows))]
const BINARY_NAME: &str = "roast";


// TODO
#[test]
fn silent() -> Result<(), String> {
    let mut cmd = Command::cargo_bin("roast").unwrap();
    let res = cmd.arg("--silent").assert().failure();

    let out = res.get_output();

    assert_is_empty!(out.stderr);
    assert_is_empty!(out.stdout);

    Ok(())
}

#[test]
fn silent_no_inputs() -> Result<(), String> {
    let mut cmd = Command::cargo_bin("roast").unwrap();
    let res = cmd.arg("--silent").assert().failure();

    let out = res.get_output();

    assert_is_empty!(out.stderr);
    assert_is_empty!(out.stdout);

    Ok(())
}

#[test]
fn help_overrides_silent() -> Result<(), String> {
    let mut cmd = Command::cargo_bin("roast").unwrap();
    let res = cmd.arg("--silent").arg("--help").assert().success();

    let out = res.get_output();
    let stdout = String::from_utf8(out.stdout.clone()).unwrap();

    assert_is_empty!(out.stderr);
    assert_contains!(stdout, &format!("Usage: {BINARY_NAME} [OPTIONS] [FILES]..."));

    Ok(())
}

#[test]
fn verbose_overrides_silent() -> Result<(), String> {
    let mut cmd = Command::cargo_bin("roast").unwrap();
    let res = cmd.arg("--verbose").arg("--silent").assert().failure();

    let out = res.get_output();

    assert_is_empty!(out.stdout);

    Ok(())
}
