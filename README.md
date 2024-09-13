# dir_diff

`dir_diff` is a small Rust program that accepts two directories as input, compares the files (based on file names) recursively, and lists the files that are not present in both directories. The full path of the files unique to each directory is displayed in the output.

## Features

- Recursively compares files in two directories based only on file names.
- Displays the full paths of files that exist only in one of the directories.
- Simple command-line interface for usage.

## Usage

### Prerequisites

- Ensure you have [Rust](https://www.rust-lang.org/) installed. You can install Rust using the following command:
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```

## Running the Program

- Clone the repository or download the project files.
- Open a terminal in the project directory.
- Run the program using the following command:
    ```bash
    cargo run <directory1> <directory2>
    ```
    Replace <directory1> and <directory2> with the paths of the directories you want to compare.

## Example

```bash
cargo run -- ./dir1 ./dir2
```

#### Output Example:

```bash
Files only in ./dir1:
./dir1/file1.txt
./dir1/subdir/file2.txt

Files only in ./dir2:
./dir2/file3.txt
```
or

```bash
./dir_diff C:/sample1 C:/sample2
```

#### Output Example

```bash
Files only in C:/sample1:
C:/sample1/file1.txt
C:/sample1/subdir/file2.txt

Files only in C:/sample2:
C:/sample2/file3.txt
```

## How it Works

1. The program takes two directory paths as input from the command line.
2. It recursively traverses both directories and gathers the file names and their full paths.
3. It compares the file names from both directories.
4. The files that exist in only one of the directories are printed along with their full paths.

## File Comparison Criteria

- The program compares files based on their file names only, not the full relative paths.
- Files that have the same name but are located in different subdirectories are treated as duplicates.
- Files unique to either directory are listed with their full paths.

## Dependencies

This project uses the walkdir crate to traverse directories recursively. It is included in the `Cargo.toml` file:

```toml
[dependencies]
walkdir = "2.5.0"
```

Installation
If you want to build the project from source:

1. Clone this repository.
2. Navigate to the project directory.
3. Build and run the project using Cargo:
    ```bash
    cargo build
    cargo run <directory1> <directory2>
    ```


