use assert_cmd::Command;
use std::fs;

// const DIFF_OK: i32 = 0;
const DIFF_FAIL: i32 = 1;
// const DIFF_FILE_NOT_FOUND: i32 = 2;
// const DIFF_FILE_LEN_DIFF: i32 = 3;

#[test]
fn test1() {
    // basic test compare 2 files and list differences
    let testfile = "tests/expected/test1.txt";
    let expected = fs::read_to_string(testfile).unwrap();
    let mut cmd = Command::cargo_bin("cmpr").unwrap();
    cmd.arg("tests/files/test1a.txt")
        .arg("tests/files/test1b.txt")
        .assert()
        .code(DIFF_FAIL)
        .stdout(expected);
}

#[test]
fn test2() {
    // basic test compare 2 files and list differences: -l option
    let testfile = "tests/expected/test2.txt";
    let expected = fs::read_to_string(testfile).unwrap();
    let mut cmd = Command::cargo_bin("cmpr").unwrap();
    cmd.arg("-l")
        .arg("tests/files/test1a.txt")
        .arg("tests/files/test1b.txt")
        .assert()
        .code(DIFF_FAIL)
        .stdout(expected);
}
