# **_finder_**

**_finder_** lets you find your files and directories (currently only on Windows).

**_finder_** was created as a grep clone. It is a **hobby** project.


## Usage
```
finder [-s | --search] <search term> [-p | --path <path>] [--debug] [--no-stream]
```

## Example
```
# find all files and directories with "test" in their name in the directory "C:\"
finder "test" -p "C:\"

# find all files and directories with "test" in their name on all drives of the system
finder -s "test"
```

## Installation
Cargo is required to build the project.

1. Clone the repository
2. Run `cargo build --release` in the repository root directory
3. The executable is located at `target/release/finder.exe`
4. Add the executable to your PATH

Voilà! You can now use `finder` in your terminal.

## CLI Flags
All flags can be changed in their order
- -s/--search: Specify the <search term> you want to search for. This flag is not needed when the term is specified as the first argument.
- -p/--path: Specify a path. This path will be considered the root of the search.
- --debug: **_finder_** will print all errors to the console.
- --no-stream: The result of the search will be only returned at the end as one block. This can have the effect, that all existing results were found but the user does not see them because **_finder_** still searches some paths.