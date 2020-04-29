use std::{process::Command, str};

use itertools::join;

#[test]
fn help_returned_by_long_flag() {
    let output = Command::new("cargo")
        .arg("run")
        .arg("--quiet")
        .arg("--")
        .arg("--help")
        .output()
        .expect("failed to execute process");
    assert!(&output.status.success());
    let stderr = str::from_utf8(&output.stderr).unwrap();
    assert!(stderr.is_empty());

    let mut stdout = str::from_utf8(&output.stdout).unwrap().lines();
    let expected = r#"Billie Thompson <billie+art-station-i-guess@billiecodes.com>
I needed to get some new wallpaper for my desktop so I made this throwaway script to download art from art station
automatically rather than manually.

USAGE:
    art-station-i-guess"#;

    assert!(&stdout.next().unwrap().starts_with("art-station-i-guess "));

    let actual_stdout = join(stdout, &'\n'.to_string());
    assert!(actual_stdout.contains(expected));
}

#[test]
fn help_returned_by_short_flag() {
    let output = Command::new("cargo")
        .arg("run")
        .arg("--quiet")
        .arg("--")
        .arg("-h")
        .output()
        .expect("failed to execute process");
    assert!(&output.status.success());
    let stderr = str::from_utf8(&output.stderr).unwrap();
    assert!(stderr.is_empty());

    let mut stdout = str::from_utf8(&output.stdout).unwrap().lines();
    let expected = r#"Billie Thompson <billie+art-station-i-guess@billiecodes.com>
I needed to get some new wallpaper for my desktop so I made this throwaway script to download art from art station
automatically rather than manually.

USAGE:
    art-station-i-guess"#;

    assert!(&stdout.next().unwrap().starts_with("art-station-i-guess "));

    let actual_stdout = join(stdout, &'\n'.to_string());
    assert!(actual_stdout.contains(expected));
}

#[test]
fn short_help_returned_when_a_wrong_message_commands_passed() {
    let output = Command::new("cargo")
        .arg("run")
        .arg("--quiet")
        .arg("--")
        .arg("--banana")
        .output()
        .expect("failed to execute process");
    assert!(!&output.status.success());
    let stdout = str::from_utf8(&output.stdout).unwrap();
    assert!(stdout.is_empty());

    let stderr = str::from_utf8(&output.stderr).unwrap();
    let expected = r#"error: Found argument '--banana' which wasn't expected, or isn't valid in this context

USAGE:
    art-station-i-guess"#;

    assert!(stderr.contains(expected));
}
