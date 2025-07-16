# Git Rusty

This project is a clone of the popular version control system, Git, built as part of the Codecrafters "Implement Your Own Git" challenge. It replicates Git’s functionality by implementing its underlying plumbing commands, which are internally invoked by porcelain commands rather than used directly. Developed in Rust, a modern and memory-safe language, this implementation ensures high performance and strong protection against low-level memory bugs.

## Table of Contents

- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)
- [Testing](#testing)
- [Project Stucture](#project-structure)
- [Technologies Used](#technologies-used)
- [Author](#author)

## Features

- Initialize an empty git repository
- Create and read blob objects
- Create and read tree objects
- Create and read commit objects
- Clone a remote repository

## Installation

```bash
git clone https://github.com/codebyred/git-rusty.git
cd git-rusty
cargo build
```

## Usage

```bash
git-rusty <COMMAND>
```

### Commands 

```
init         Initialize a new, empty Git repository
cat-file     Provide content or type and size information for repository objects
hash-object  Compute object ID and optionally create a blob from a file
ls-tree      List the contents of a tree object
write-tree   Write a tree object from the current index
commit-tree  Create a commit object
clone        Clone a repository into a new directory
help         Print this message or the help of the given subcommand(s)
```

## Testing

This project includes integration tests to ensure key functionalities work correctly.

### Integration Tests

- `write_tree_test` – Tests the `write-tree` command functionality.
- `commit_tree_test` – Tests the `commit-tree` command functionality.

### Running Tests

Run all tests using:

```bash
cargo test
```

## Project Structure

```
git-rusty/
├── .gitignore
├── Cargo.lock
├── Cargo.toml
├── LICENSE
├── README.md
└── src/
    ├── main.rs              # Main entry point
    ├── object.rs            # Git Object-related functionality
    └── commands/            # Git command implementations
        ├── mod.rs           # Module declarations for commands
        ├── cat_file.rs      # Implements 'cat-file' command
        ├── clone.rs         # Implements 'clone' command
        ├── commit_tree.rs   # Implements 'commit-tree' command
        ├── hash_object.rs   # Implements 'hash-object' command
        ├── init.rs          # Implements 'init' command
        ├── ls_tree.rs       # Implements 'ls-tree' command
        └── write_tree.rs    # Implements 'write-tree' command
```

## Technologies Used

- **Rust** – Programming language
- **clap** – Command-line argument parsing
- **reqwest** – HTTP requests (used with blocking feature)
- **flate2** – Compression and decompression (e.g. zlib for Git objects)
- **anyhow** – Simple error handling library
- **thiserror** – Deriving custom error types
- **bytes** – Efficient buffer management
- **hex** – Hex encoding and decoding utilities
- **sha1** – SHA-1 hashing (for Git object IDs)
- **tempfile** – Temporary file and directory utilities
- **regex** – Regular expressions library
- **assert_cmd** – Testing command line applications


## Author

[codebyred](https://github.com/codebyred)

