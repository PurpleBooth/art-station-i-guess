use std::{env, fs, io, process::Command, str};
use tempfile::tempdir;

fn calculate_cargo_toml_path() -> String {
    env::current_exe()
        .unwrap()
        .parent()
        .and_then(std::path::Path::parent)
        .and_then(std::path::Path::parent)
        .and_then(std::path::Path::parent)
        .map(|x| x.join("Cargo.toml"))
        .unwrap()
        .to_str()
        .unwrap()
        .to_string()
}

#[test]
fn some_images_are_downloaded() {
    let temp_dir = tempdir().expect("failed to create test dir");
    let output = Command::new("cargo")
        .current_dir(&temp_dir)
        .arg("run")
        .arg("--quiet")
        .arg("--manifest-path")
        .arg(calculate_cargo_toml_path())
        .arg("--")
        .arg("manuelmartinart")
        .output()
        .expect("failed to execute process");

    let stdout = str::from_utf8(&output.stdout)
        .expect("Failed to convert stdout to a string, is it valid UTF-8?");
    let stderr = str::from_utf8(&output.stderr)
        .expect("Failed to convert stderr to a string, is it valid UTF-8?");

    let expected_prefix = "art-station-i-guess ";
    assert!(
        stdout.contains("successfully wrote to "),
        "Expected stdout to start with {:?}, instead got stdout: {:?} stderr: {:?}",
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

    let images = fs::read_dir(&temp_dir)
        .expect("failed to read temp dir")
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()
        .expect("failed to read temp dir")
        .iter()
        .filter(|x| {
            x.as_os_str()
                .to_str()
                .map_or(false, |f| f.ends_with(".jpg"))
        })
        .count();

    assert_ne!(
        images, 0,
        "Expected multiple images to have been downloaded, found {:?}",
        images
    )
}
