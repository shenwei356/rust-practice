use std::fs;

use assert_cmd::cargo;
use predicates::prelude::*;

#[test]
fn runs() {
    let mut cmd = cargo::cargo_bin_cmd!("echor");
    cmd.arg("hello").assert().success();
}

#[test]
fn test_unicode() {
    let file = "tests/unicode.txt";
    let expected = fs::read_to_string(file).unwrap();
    let mut cmd = cargo::cargo_bin_cmd!("echor");
    cmd.args(vec!["Hello,", "沈伟!"])
        .assert()
        .success()
        .stdout(expected);
}

type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn wrong_separator() -> TestResult {
    let mut cmd = cargo::cargo_bin_cmd!("echor");
    cmd.arg("-s")
        .assert()
        .failure()
        .stderr(predicate::str::contains("error:"));
    Ok(())
}

fn run(args: &[&str], expected_file: &str) -> TestResult {
    let expected = fs::read_to_string(expected_file).unwrap();
    let mut cmd = cargo::cargo_bin_cmd!("echor");
    cmd.args(args).assert().success().stdout(expected);
    Ok(())
}

#[test]
fn test1() -> TestResult {
    run(&["Hello World!"], "tests/hello1.txt")
}
