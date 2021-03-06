use std::{process::Command, str};

#[test]
fn version_returned_by_long_flag() {
    let output = Command::new("cargo")
        .arg("run")
        .arg("--quiet")
        .arg("--")
        .arg("--version")
        .output()
        .expect("failed to execute process");

    let stdout = str::from_utf8(&output.stdout)
        .expect("Failed to convert stdout to a string, is it valid UTF-8?");
    let stderr = str::from_utf8(&output.stderr)
        .expect("Failed to convert stderr to a string, is it valid UTF-8?");

    let expected_name = "art-station-i-guess";
    assert!(
        stdout.contains(expected_name),
        "Expected stdout to contain {:?}, instead got stdout: {:?} stderr: {:?}",
        expected_name,
        stdout,
        stderr
    );
    assert!(
        stdout.ends_with('\n'),
        "Expected stdout to end with a new line, instead got stdout: {:?} stderr: {:?}",
        stdout,
        stderr
    );

    assert!(
        stderr.is_empty(),
        "Expected stderr to be empty, instead got stdout: {:?} stderr: {:?}",
        stdout,
        stderr
    );
    assert!(
        &output.status.success(),
        "Expected command to run successfully, instead got {:?}",
        output.status.code()
    );
}

#[test]
fn version_returned_by_short_flag() {
    let output = Command::new("cargo")
        .arg("run")
        .arg("--quiet")
        .arg("--")
        .arg("-V")
        .output()
        .expect("failed to execute process");

    let stderr = str::from_utf8(&output.stderr)
        .expect("Failed to convert stdout to a string, is it valid UTF-8?");
    let stdout = str::from_utf8(&output.stdout)
        .expect("Failed to convert stderr to a string, is it valid UTF-8?");

    assert!(
        stderr.is_empty(),
        "Expected stderr to be empty, instead got stdout: {:?} stderr: {:?}",
        stdout,
        stderr
    );

    let expected_prefix = "art-station-i-guess";
    assert!(
        stdout.contains(expected_prefix),
        "Expected stdout to contain {:?}, instead got stdout: {:?} stderr: {:?}",
        expected_prefix,
        stdout,
        stderr
    );
    assert!(
        stdout.ends_with('\n'),
        "Expected stdout to end with a new line, instead got stdout: {:?} stderr: {:?}",
        stdout,
        stderr
    );

    assert!(
        &output.status.success(),
        "Expected command to run successfully, instead got {:?}",
        output.status.code()
    );
}
