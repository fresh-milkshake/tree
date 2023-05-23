# "Tree" in Rust

This is a simple command-line application named "tree" developed in Rust. The purpose of this application is to print the structure of a directory as a tree.

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)

## Installation

To install this project, you'll first need to clone it from the GitHub repository. You can do this by executing the following command in your terminal:

```bash
git clone https://github.com/fresh-milkshake/tree
```

Once you've cloned the repository, navigate into the directory:

```bash
cd tree
```

## Build

You can build the project using cargo (Rust's package manager and build system) with the following command:

```bash
cargo build --release
```

## Usage

This application is intended to be run from the command line. You can execute the program with cargo run, followed by the path of the directory you want to visualize and the depth of directory levels you want to visualize. 

The application takes two arguments:
- The first one is the path of the directory you want to visualize.
- The second one is the depth of directory levels you want to visualize.

```bash
cargo run <path> <depth>
```

For example:

```bash
cargo run ./src 2
```

This will print the tree structure of the `src` directory up to two levels deep.

## Contribution

Feel free to create a fork of the project, make your changes, and then create a pull request. Make sure to follow the existing coding style.

## License

This project is licensed under the MIT License. For more informaton, please refer to the `LICENSE` file in the reposi

For more information, please refer the project's [LICENSE file](https://github.com/fresh-milkshake/tree/LICENSE.txt)