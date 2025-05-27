use std::fs::{create_dir_all, write};
use std::process::Command;
use anyhow::Context;
use tempfile::tempdir;
use std::env;

#[test]
fn test_write_tree_hash() -> anyhow::Result<()> {
    // Create a safe, isolated temporary directory
    let temp_dir = tempdir().context("creating tmp dir")?;
    let path = temp_dir.path();

    // Change to the temporary directory so cargo run doesn't touch project root
    env::set_current_dir(&path).context("setting curr path env")?;

    let binary = assert_cmd::cargo::cargo_bin("git-rusty");

    // Run the init command from within the temp directory
    let status = Command::new(&binary)
        .args(["init"])
        .status()
        .context("Failed to run `init`")?;
    assert!(status.success());

    // Create test files
    write("test_file_1.txt", "hello world").context("writing to test_file_1")?;
    create_dir_all("test_dir_1").context("creating test_dir_1")?;
    write("test_dir_1/test_file_2.txt", "hello world").context("writing to test_file_2")?;
    create_dir_all("test_dir_2").context("creating test_dir_2")?;
    write("test_dir_2/test_file_3.txt", "hello world").context("writing to test_file_3")?;

    // Run write-tree and get output
    let output = Command::new(&binary)
        .args(["write-tree"])
        .output()
        .context("Failed to run `write-tree`")?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let hash = stdout.trim();

    let ls_tree_output = Command::new(&binary)
        .args(["ls-tree", hash])
        .output()
        .context("Failed to run ls-tree")?;

    println!("{}", String::from_utf8_lossy(&ls_tree_output.stdout));

    // Verify hash
    assert_eq!(
        hash,
        "4b825dc642cb6eb9a060e54bf8d69288fbee4904",
        "Tree hash did not match expected"
    );

    Ok(())
}
