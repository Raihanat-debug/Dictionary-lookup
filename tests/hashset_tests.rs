use std::io::Write;
use std::process::{Command, Stdio};

#[test]
fn test_sample() {
    let input = b"5
apple
banana
orange
grape
pear
4
banana
kiwi
apple
melon
";

    let mut child = Command::new("cargo")
        .args(["run", "--bin", "hashset"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to run hashset");

    child.stdin.as_mut().unwrap().write_all(input).unwrap();

    let output = child.wait_with_output().unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert_eq!(
        stdout.trim(),
        "YES
NO
YES
NO"
    );
}

#[test]
fn test_case_insensitive() {
    let input = b"3
Apple
Banana
Orange
3
apple
BANANA
orange
";

    let mut child = Command::new("cargo")
        .args(["run", "--bin", "hashset"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to run hashset");

    child.stdin.as_mut().unwrap().write_all(input).unwrap();

    let output = child.wait_with_output().unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert_eq!(
        stdout.trim(),
        "YES
YES
YES"
    );
}

#[test]
fn test_not_found() {
    let input = b"2
cat
dog
3
bird
cow
fish
";

    let mut child = Command::new("cargo")
        .args(["run", "--bin", "hashset"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to run hashset");

    child.stdin.as_mut().unwrap().write_all(input).unwrap();

    let output = child.wait_with_output().unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert_eq!(
        stdout.trim(),
        "NO
NO
NO"
    );
}