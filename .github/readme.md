# Brainfuck-2D (bf2d)


`bf2d` is an enhanced interpreter for the Brainfuck programming language, written in Rust. It extends the classic Brainfuck language by introducing a two-dimensional memory space, enabling more complex and visually intuitive programming patterns.

## Features
- **Two-Dimensional Memory Mapping**: Memory in `bf2d` is arranged in a square grid, allowing for both horizontal and vertical traversal.
- **Vertical Navigation Commands**: New commands `^` and `v` are introduced for moving up and down in the memory grid.
- **Memory Print Command**: The `!` command outputs the entire memory grid, providing a comprehensive view of the current program state.
- **Classic Brainfuck Interpreter**: Traditional brainfuck programs are supported

## Installation

`bf2d` offers two installation methods: using pre-compiled binaries or building from source. 

### Using Pre-Compiled Binaries
1. Download the latest binary for your operating system from the [Releases page](https://github.com/keitannunes/brainfuck-2d/releases).
2. Unzip the downloaded file to a directory of your choice.
3. Add the directory to your system's PATH environment variable to access `bf2d` from any command line.

### Building from Source
If you prefer to build the interpreter from the source, follow these steps:

1. Ensure you have Rust installed on your machine. If not, download and install it from [https://www.rust-lang.org/](https://www.rust-lang.org/).
2. Clone the `brainfuck-2d` repository:
   ```
   git clone https://github.com/keitannunes/brainfuck-2d
   ```
3. Navigate to the cloned directory:
   ```
   cd brainfuck-2d
   ```
4. Build the project using Cargo:
   ```
   cargo build --release
   ```
   
### Usage

```
Usage: bf2d [OPTIONS] [FILENAME]

Arguments:
  [FILENAME]

Options:
  -d <DIMENSION>      Dimension of memory [default: 8]
  -v                  View memory dump
  -h, --help          Print help
```


#### Using Standard Input
In addition to specifying the path to a Brainfuck file, `bf2d` also supports receiving Brainfuck code through standard input (stdin).

To use standard input, simply pipe the Brainfuck code into `brainfuck_2d` without specifying a file path. For example:
```
echo '++++++[>++++++++<-]>.' | bf2d
```

This command will interpret and execute the code provided directly via the command line.

**Note:** programs received from stdin does not support the `,` command 

## Commands
- `>`: Move right in the memory grid (will wrap to next line).
- `<`: Move left in the memory grid (will wrap to previous line).
- `+`: Increment the memory cell at the current position.
- `-`: Decrement the memory cell at the current position.
- `.`: Output the character signified by the cell at the current position.
- `,`: Input a character and store it in the cell at the current position.
- `[`: Jump past the matching `]` if the cell at the current position is 0.
- `]`: Jump back to the matching `[` if the cell at the current position is nonzero.
- `^`: Move up in the memory grid.
- `v`: Move down in the memory grid.
- `!`: Print the entire memory grid ('\0' will be printed as a space).

## Example
Here's a simple *hello!* world program in Brainfuck-2D:

```
>>>>>+++++++[>+++++<-]<v+++++++[>+++++<-]<v+++++++[>+++++>+++++>+++++>+++++><<<<<-]<v+++++++[>+++++>>>>+++++<<<<<-]<v+++++++[>+++++>>>>+++++<<<<<-]<v+++++++[>+++++>>>>+++++<<<<<-]v>+++++++[<+++++>>>>+++++<<<-]>>>>++++++++++[>++++++++++<-]>+<++++++++++++[>>+++++++++<<-]!>>.>+++++++++++[>++++++++++<-]>+.>+++++++++++[>+++<-]>.
```


## License
`brainfuck_2d` is licensed under [GPLv3](https://www.gnu.org/licenses/gpl-3.0.ja.html).
