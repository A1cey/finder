# finder
A simple grep clone for finding files and directories on Windows.

## Usage
```
finder [-s | --search] <search term> [-p | --p <path>]
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