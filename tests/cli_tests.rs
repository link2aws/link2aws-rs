//! Tests the command line interface of the `link2aws` binary.

use assert_cmd::{Command, assert::Assert};

const VALID_ARN_1: &str = "arn:aws:s3:::111";
const VALID_ARN_1_LINK: &str = "https://s3.console.aws.amazon.com/s3/buckets/111";

const VALID_ARN_2: &str = "arn:aws:s3:::222";
const VALID_ARN_2_LINK: &str = "https://s3.console.aws.amazon.com/s3/buckets/222";

const INVALID_ARN: &str = "this-is-not-an-arn";

fn verify_help(assert: Assert) {
    let assert = assert.success();
    let stdout = String::from_utf8(assert.get_output().stdout.clone()).unwrap();
    assert!(stdout.contains("Usage:"));
    assert!(stdout.contains("Arguments:"));
    assert!(stdout.contains("Options:"));
}

#[test]
fn dash_dash_help() {
    let mut cmd = Command::cargo_bin("link2aws").unwrap();
    let assert = cmd.arg("--help").assert();
    verify_help(assert);
}

#[test]
fn dash_h() {
    let mut cmd = Command::cargo_bin("link2aws").unwrap();
    let assert = cmd.arg("-h").assert();
    verify_help(assert);
}

fn verify_version(assert: Assert) {
    let assert = assert.success();
    let stdout = String::from_utf8(assert.get_output().stdout.clone()).unwrap();
    assert!(stdout.contains("link2aws"));
}

#[test]
fn dash_dash_version() {
    let mut cmd = Command::cargo_bin("link2aws").unwrap();
    let assert = cmd.arg("--version").assert();
    verify_version(assert);
}

#[test]
fn dash_capital_v() {
    let mut cmd = Command::cargo_bin("link2aws").unwrap();
    let assert = cmd.arg("-V").assert();
    verify_version(assert);
}

#[test]
fn one_positional_arn_success() {
    let mut cmd = Command::cargo_bin("link2aws").unwrap();
    let assert = cmd.arg(VALID_ARN_1).assert().success();
    let stdout = String::from_utf8(assert.get_output().stdout.clone()).unwrap();
    assert_eq!(stdout, format!("{VALID_ARN_1_LINK}\n"));
}

#[test]
fn two_positional_arns_success() {
    let mut cmd = Command::cargo_bin("link2aws").unwrap();
    let assert = cmd.arg(VALID_ARN_1).arg(VALID_ARN_2).assert().success();
    let stdout = String::from_utf8(assert.get_output().stdout.clone()).unwrap();
    assert_eq!(stdout, format!("{VALID_ARN_1_LINK}\n{VALID_ARN_2_LINK}\n"));
}

#[test]
fn two_positional_arns_one_valid_and_one_invalid_dash_q() {
    let mut cmd = Command::cargo_bin("link2aws").unwrap();
    let assert = cmd
        .arg(VALID_ARN_1)
        .arg(INVALID_ARN)
        .arg("-q")
        .assert()
        .failure()
        .code(1);
    let stdout = String::from_utf8(assert.get_output().stdout.clone()).unwrap();
    assert_eq!(stdout, format!("{VALID_ARN_1_LINK}\n"));
    let stderr = String::from_utf8(assert.get_output().stderr.clone()).unwrap();
    assert_eq!(stderr, "");
}

#[test]
fn two_positional_arns_one_valid_and_one_invalid_dash_dash_quiet() {
    let mut cmd = Command::cargo_bin("link2aws").unwrap();
    let assert = cmd
        .arg(VALID_ARN_1)
        .arg(INVALID_ARN)
        .arg("--quiet")
        .assert()
        .failure()
        .code(1);
    let stdout = String::from_utf8(assert.get_output().stdout.clone()).unwrap();
    assert_eq!(stdout, format!("{VALID_ARN_1_LINK}\n"));
    let stderr = String::from_utf8(assert.get_output().stderr.clone()).unwrap();
    assert_eq!(stderr, "");
}

#[test]
fn two_positional_arns_one_valid_and_one_invalid() {
    let mut cmd = Command::cargo_bin("link2aws").unwrap();
    let assert = cmd
        .arg(VALID_ARN_1)
        .arg(INVALID_ARN)
        .assert()
        .failure()
        .code(1);
    let stdout = String::from_utf8(assert.get_output().stdout.clone()).unwrap();
    assert_eq!(stdout, format!("{VALID_ARN_1_LINK}\n"));
    let stderr = String::from_utf8(assert.get_output().stderr.clone()).unwrap();
    assert_eq!(
        stderr,
        format!("link2aws: \"this-is-not-an-arn\": ARN is malformed\n")
    );
}

#[test]
fn one_stdin_arn_success() {
    let mut cmd = Command::cargo_bin("link2aws").unwrap();
    let assert = cmd
        .arg("--stdin")
        .write_stdin(VALID_ARN_1)
        .assert()
        .success();
    let stdout = String::from_utf8(assert.get_output().stdout.clone()).unwrap();
    assert_eq!(stdout, format!("{VALID_ARN_1_LINK}\n"));
}

#[test]
fn two_stdin_arns_success() {
    let mut cmd = Command::cargo_bin("link2aws").unwrap();
    let assert = cmd
        .arg("--stdin")
        .write_stdin(format!("{VALID_ARN_1}\n{VALID_ARN_2}"))
        .assert()
        .success();
    let stdout = String::from_utf8(assert.get_output().stdout.clone()).unwrap();
    assert_eq!(stdout, format!("{VALID_ARN_1_LINK}\n{VALID_ARN_2_LINK}\n"));
}

#[test]
fn two_stdin_arns_with_trailing_newline_success() {
    let mut cmd = Command::cargo_bin("link2aws").unwrap();
    let assert = cmd
        .arg("--stdin")
        .write_stdin(format!("{VALID_ARN_1}\n{VALID_ARN_2}\n"))
        .assert()
        .success();
    let stdout = String::from_utf8(assert.get_output().stdout.clone()).unwrap();
    assert_eq!(stdout, format!("{VALID_ARN_1_LINK}\n{VALID_ARN_2_LINK}\n"));
}

#[test]
fn two_stdin_arns_one_valid_and_one_invalid() {
    let mut cmd = Command::cargo_bin("link2aws").unwrap();
    let assert = cmd
        .arg("--stdin")
        .write_stdin(format!("{VALID_ARN_1}\n{INVALID_ARN}\n"))
        .assert()
        .failure()
        .code(1);
    let stdout = String::from_utf8(assert.get_output().stdout.clone()).unwrap();
    assert_eq!(stdout, format!("{VALID_ARN_1_LINK}\n"));
    let stderr = String::from_utf8(assert.get_output().stderr.clone()).unwrap();
    assert_eq!(
        stderr,
        "link2aws: \"this-is-not-an-arn\": ARN is malformed\n"
    );
}

#[test]
fn two_stdin_arns_one_valid_and_one_invalid_dash_dash_quiet() {
    let mut cmd = Command::cargo_bin("link2aws").unwrap();
    let assert = cmd
        .arg("--stdin")
        .arg("--quiet")
        .write_stdin(format!("{VALID_ARN_1}\n{INVALID_ARN}\n"))
        .assert()
        .failure()
        .code(1);
    let stdout = String::from_utf8(assert.get_output().stdout.clone()).unwrap();
    assert_eq!(stdout, format!("{VALID_ARN_1_LINK}\n"));
    let stderr = String::from_utf8(assert.get_output().stderr.clone()).unwrap();
    assert_eq!(stderr, "");
}
