use assert_cmd::Command;
use std::fs;


#[test]
fn test1() {
    // basic test compare 2 files and list differences
    let testfile = "tests/expected/test1.txt";
    let expected = fs::read_to_string(testfile).unwrap();
    let mut cmd = Command::cargo_bin("cmpr").unwrap();
    cmd.arg("files/test1a.txt")
        .arg("files/testt1b.txt")
        .assert()
        .success()
        .stdout(expected);
}
