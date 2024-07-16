# xfetch

**xfetch** is an essential component of **[Cortex](https://github.com/neomannskar/cortex)**, implemented as a standalone console application. **xfetch** (_cortex-fetch_) serves as a fetch tool designed to retrieve local files and directories for use across different projects.

## Installation and Building

**xfetch** is a lightweight console application written in **Rust**. To get started, clone this repository and navigate to the newly created directory. Then, use the following commands to build and install **xfetch**.

### Building with Cargo

Ensure you have [Rust and Cargo installed](https://www.rust-lang.org/tools/install).

```bash
git clone https://github.com/neomannskar/xfetch.git
cd xfetch
cargo build --release
```

This will create an executable in the `target/release` directory. You can add this executable to your system's PATH variable for easy access.

### Adding to PATH

On Unix-like systems (Linux, macOS):

```bash
export PATH=$PATH:/path/to/xfetch/target/release
```

On Windows (PowerShell):

```powershell
$env:Path += ";C:\path\to\xfetch\target\release"
```

Replace `/path/to/xfetch/target/release` with the actual path to the `xfetch` executable.

## How to Use the Application

**xfetch** accepts command-line arguments to perform various actions. Begin by invoking the program in your terminal:

```bash
xfetch
```

Running **xfetch** without any arguments prints out usage information.

### Creating Files and Directories

To create a file or directory, use the `create` subcommand.

### Creating a File

```bash
xfetch create path/to/file.txt
```

### Creating a Directory

```bash
xfetch create path/to/directory
```

### Importing Files and Directories

To import a file or directory, use the `import` subcommand. Specify the source path and optionally the destination directory. If no destination is provided, the current directory is used.

### Importing a File

```bash
xfetch import path/to/source/file.txt
```

### Importing a Directory

```bash
xfetch import path/to/source/directory
```

You can also specify a destination:

```bash
xfetch import path/to/source/file.txt path/to/destination
```

_Note: **xfetch** implements safeguards against accidental overwrites, requiring user confirmation._

## Future Features

### 1. Path Abstraction
