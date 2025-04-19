use assert_cmd::Command;
use assertables::{assert_contains, assert_is_empty};

#[test]
fn buffered_mode() -> Result<(), String> {
    let mut cmd = Command::cargo_bin("roast").unwrap();
    let res = cmd
        .write_stdin("{\"test\": 2}")
        .arg("--buffered")
        .arg("--lineEnding")
        .arg("lf")
        .assert()
        .success();

    let out = res.get_output();
    let stdout = String::from_utf8(out.stdout.clone()).unwrap();

    assert_is_empty!(out.stderr);
    assert_eq!(stdout, "{\n\t\"test\": 2\n}\n");

    Ok(())
}

#[test]
fn buffered_mode_newlines_ignored() -> Result<(), String> {
    let mut cmd = Command::cargo_bin("roast").unwrap();
    let res = cmd
        .write_stdin("{\"test\": 2}\n\n\n\n")
        .arg("-b")
        .arg("--lineEnding")
        .arg("lf")
        .assert()
        .success();

    let out = res.get_output();
    let stdout = String::from_utf8(out.stdout.clone()).unwrap();

    assert_is_empty!(out.stderr);
    assert_eq!(stdout, "{\n\t\"test\": 2\n}\n");

    Ok(())
}

#[test]
fn buffered_mode_invalid_input() -> Result<(), String> {
    let mut cmd = Command::cargo_bin("roast").unwrap();
    let res = cmd
        .write_stdin("asdf")
        .arg("-b")
        .arg("--lineEnding")
        .arg("lf")
        .assert()
        .failure();

    let out = res.get_output();
    let stderr = String::from_utf8(out.stderr.clone()).unwrap();

    assert_is_empty!(out.stdout);
    assert_contains!(stderr, "Error ParseError");

    Ok(())
}
