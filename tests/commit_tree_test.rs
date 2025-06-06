use std::env;
use anyhow::Context;
use assert_cmd::Command;
use tempfile::tempdir;
use std::fs::{create_dir_all, write};
use regex::Regex;

#[test]
fn test_commit_tree() -> anyhow::Result<()> {
    let temp_dir = tempdir().with_context(|| format!("error at create temp dir"))?;

    let temp_dir_path = temp_dir.path();

    env::set_current_dir(temp_dir_path)
        .with_context(|| format!("error at set env current directory"))?;

    let binary = assert_cmd::cargo::cargo_bin("git-rusty");

    Command::new(&binary)
        .args(["init"])
        .assert()
        .success();

    write("test.txt", "hello world")
        .with_context(|| format!("error at write to test.txt"))?;


    let git_write_tree_output = Command::new(&binary)
        .args(["write-tree"])
        .output()
        .with_context(|| format!("error at git write-tree"))?;

    let git_write_tree_output= String::from_utf8_lossy(&git_write_tree_output.stdout);
    let tree_hash = git_write_tree_output.trim();

    assert!(is_valid_git_hash(&tree_hash), "Invalid tree hash: {}", tree_hash);

    let git_commit_tree_output = Command::new(&binary)
        .args(["commit-tree", tree_hash, "-m", "Initial commit"])
        .output()
        .with_context(|| format!("error at git write-tree"))?;

    let error = String::from_utf8_lossy(&git_commit_tree_output.stderr);
    println!("{}", error.trim());
    let git_commit_tree_output = String::from_utf8_lossy(&git_commit_tree_output.stdout);
    
    let commit_hash = git_commit_tree_output.trim();
    assert!(is_valid_git_hash(&commit_hash), "Invalid commit hash: {}", commit_hash);

    write("test.txt", "hello world 2")
        .with_context(|| format!("error at write to test.txt"))?;


    let git_write_tree_output = Command::new(&binary)
        .args(["write-tree"])
        .output()
        .with_context(|| format!("error at git write-tree"))?;


    let git_write_tree_output= String::from_utf8_lossy(&git_write_tree_output.stdout);
    let tree_hash = git_write_tree_output.trim();
    assert!(is_valid_git_hash(&tree_hash), "Invalid tree hash: {}", tree_hash);

    Ok(())
}

fn is_valid_git_hash(s: &str) -> bool {
    let re = Regex::new(r"^[0-9a-f]{40}$").unwrap();
    re.is_match(s)
}
