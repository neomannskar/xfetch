# xfetch

An important part of **[Cortex](https://github.com/neomannskar/cortex)** implemented as a standalone console application. **xfetch (cortex-fetch)** is a fetch tool which retrieves local files and directories for use in different projects.

## Installation and Building

**xfetch** is a small console application written in pure **C++17**. It has no other dependencies and can simply be compiled using your favorite C++ compiler, just make sure that the version supports the **C++17** standard or later.

Start by cloning down this repository. Then navigate into the newly created directory and choose the appropriate command below:

### GCC

```bash
g++ -std=c++17 -o ./bin/xfetch ./src/main.cpp
```

### Clang

```bash
clang++ -std=c++17 -o ./bin/xfetch ./src/main.cpp
```

After the application has been successfully compiled, simply add the it to your system variables path.

## How to use the application

**xfetch** takes some commandline arguments as input and executes different actions based on those arguments. Start by invoking the program in the console.

```bash
xfetch
```

If you invoke the program without providing any arguments it will simply print out the usage (path to the executable).

Now provide **xfetch** a type-specifier, the type is either a file (`file`) or a directory (`dir`).

### File

```bash
xfetch file
```

### Directory

```bash
xfetch dir
```

Alternatively you can use the type-specifier `folder` for directories if that suits you better.

```bash
xfetch folder
```

Finally specify the source path to the content you want to import. E.g:

```bash
xfetch file C:\dev\example\file.txt
```

_Note that **xfetch** takes a source path as input and imports the content into the directory in which the application is currently running!_

## Important

**xfetch** protects against accidental overwriting and needs the user's confirmation when trying to overwrite files and directories.
