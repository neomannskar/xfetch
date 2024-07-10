# xfetch

**xfetch** is an essential component of **[Cortex](https://github.com/neomannskar/cortex)**, implemented as a standalone console application. **xfetch** (_cortex-fetch_) serves as a fetch tool designed to retrieve local files and directories for use across different projects.

## Installation and Building

**xfetch** is a lightweight console application written in pure **C++17**. It has no external dependencies and can be compiled using your preferred C++ compiler, ensuring it supports the **C++17** standard or later.

To get started, clone this repository and navigate to the newly created directory. Then, choose the appropriate command based on your compiler:
### GCC

```bash
g++ -std=c++17 -o ./bin/xfetch ./src/main.cpp
```
### Clang

```bash
clang++ -std=c++17 -o ./bin/xfetch ./src/main.cpp
```

Once successfully compiled, add **xfetch** to your system's PATH variable.

## How to Use the Application

**xfetch** accepts command-line arguments to perform various actions. Begin by invoking the program in your terminal:

```bash
xfetch
```

Running **xfetch** without any arguments prints out usage information (the path to the executable).

Specify a type-specifier (`file` or `dir`) to indicate whether you are fetching a file or a directory.

### File

```bash
xfetch file
```

### Directory

```bash
xfetch dir
```

Alternatively, you can use `folder` as the type-specifier for directories if preferred.

```bash
xfetch folder
```

Finally, provide the source path of the content you wish to import. For example:

```bash
xfetch file C:\dev\example\file.txt
```

_Note: **xfetch** imports content into the current directory of the application and implements safeguards against accidental overwrites, requiring user confirmation._

## Future features

### 1. Path abstraction
