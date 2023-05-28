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

#[test]
fn test3() {
    // basic test compare 2 files and list differences: -h option
    let testfile = "tests/expected/test3.txt";
    let expected = fs::read_to_string(testfile).unwrap();
    let mut cmd = Command::cargo_bin("cmpr").unwrap();
    cmd.arg("-h")
        .arg("tests/files/test1a.txt")
        .arg("tests/files/test1b.txt")
        .assert()
        .code(DIFF_FAIL)
        .stdout(expected);
}

// test4 test -c
// test5 test EOF message file1
// test6 test EOF message file2
// test7 test correct addr for file > 100 bytes
// test8 test file does not exist file1
// test9 test skip option
// test10 test invalid skip option
// test11 test -c with control chars if implement
// test12 test with stdin if implement


// test9 
