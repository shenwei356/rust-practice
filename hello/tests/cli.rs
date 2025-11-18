use assert_cmd::Command as Cmd;
use std::process::Command;

#[test]
fn works() {
    assert!(true);
}

#[test]
fn run_ls() {
    let mut cmd = Command::new("ls");
    let res = cmd.output();
    assert!(res.is_ok());
}

#[test]
fn runs_echo() {
    let mut cmd = Cmd::cargo_bin("hello").unwrap();
    cmd.assert().success();
}

#[test]
fn true_ok() {
    let mut cmd = Cmd::cargo_bin("true").unwrap();
    cmd.assert().success();
}

#[test]
fn false_not_ok() {
    let mut cmd = Cmd::cargo_bin("false").unwrap();
    cmd.assert().failure();
}

#[test]
fn runs() {
    let mut cmd = Cmd::cargo_bin("hello").unwrap();
    cmd.assert().success().stdout("Hello, world!\n");
}
